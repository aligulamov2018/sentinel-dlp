//! Каноническое представление `Decision` и `InspectionResult` (CHG-0008).
//!
//! Контракт — docs/architecture/SENTINEL_CORE_CANONICAL_FORM.md (утверждён владельцем
//! 2026-11-13, решения CD-1…CD-8). Назначение — проверка детерминизма: сравнение с рукописными
//! эталонами и побайтовое сравнение запусков (SENTINEL_CORE_DESIGN.md §6.5, TC-CORE-15). Это не
//! формат CLI, IPC, бандла или событий.
//!
//! Свойства кодировщика (CF-15, CD-6):
//! - полный: кодирует любое значение типа и не паникует;
//! - не проверяет семантические инварианты и не нормализует вход: порядок и повторы элементов
//!   массивов сохраняются, строки выводятся точно (CF-5, CF-12); проверки контракта — отдельно
//!   (`InspectionResult::check_contract`, тесты PDP);
//! - только кодирование; разбора канонических байтов нет.
//!
//! Модуль доступен только с cargo-функцией `canonical` (CF-16, CD-7). Функция — настройка сборки
//! против случайного использования, а не граница безопасности: любой crate графа зависимостей
//! может её включить. Канонические байты не предназначены для событий, журналов, SIEM и вывода
//! пользователю (контракт, раздел 10).
//!
//! Пример:
//!
//! ```
//! use sentinel_core_types::canonical::encode_inspection_result;
//! use sentinel_core_types::*;
//!
//! let r = InspectionResult {
//!     status: InspectionStatus::NoMatch,
//!     reasons: vec![],
//!     text_coverage: TextCoverage::Complete,
//!     structure_enumeration: StructureEnumeration::Complete,
//!     classification: Classification::Confirmed,
//!     uninspected_parts: vec![],
//!     detections: vec![],
//!     file_class: FileClass::new("TXT"),
//!     digest: ContentDigest::not_computed(DigestStatus::NotRequested),
//!     engine: EngineInfo {
//!         core_version: "0.1.0".into(),
//!         detector_set_version: "ds-1".into(),
//!         limits_profile_id: "lp-1".into(),
//!     },
//!     inspection_origin: InspectionOrigin::Core,
//! };
//! let bytes = encode_inspection_result(&r);
//! assert!(bytes.starts_with(b"{\"canonical_form\":\"sentinel-core/inspection-result/canonical/v1\""));
//! ```
//!
//! Предварительное решение не кодируется (§5: `ProvisionalDecision` не сериализуется). Код ошибки
//! указан справочно: rustdoc на stable его не проверяет, поэтому причина отказа дополнительно
//! проверяется сценарием C-22b (компиляция пробы с проверкой `E0308`):
//!
//! ```compile_fail,E0308
//! use sentinel_core_types::canonical::encode_decision;
//! use sentinel_core_types::ProvisionalDecision;
//! fn host(p: &ProvisionalDecision) -> Vec<u8> {
//!     encode_decision(p) // ошибка типов: ProvisionalDecision — не Decision
//! }
//! ```

use crate::{
    Action, ActionCertainty, ActionSource, ActionSourceRef, AmbiguityReason, CapabilityCheck, Classification,
    ContentDigest, DataLevel, Decision, DecisionBasis, DetectionResult, DeviationInput, DigestStatus,
    DigestTrust, EngineInfo, Explanation, Fallback, InspectionOrigin, InspectionResult, InspectionStatus,
    OrderBasis, PartKind, ReasonCode, RuleId, RuleTrace, StructureEnumeration, TextCoverage, Truth,
    UninspectedPart, Verdict, VerificationMode,
};

/// Идентификатор канонической формы `Decision` (CF-13).
pub const DECISION_CANONICAL_FORM: &str = "sentinel-core/decision/canonical/v1";

/// Идентификатор канонической формы `InspectionResult` (CF-13).
pub const INSPECTION_RESULT_CANONICAL_FORM: &str = "sentinel-core/inspection-result/canonical/v1";

/// Версия канонического представления `InspectionResult` (CD-4, вариант «а»). Поле в Rust-типе
/// не добавляется; ИТ-22 для самого типа и других представлений остаётся открытым.
pub const INSPECTION_RESULT_SCHEMA_VERSION: u32 = 1;

/// Каноническое представление финального решения (CF-1…CF-17).
pub fn encode_decision(decision: &Decision) -> Vec<u8> {
    let mut w = Writer::default();
    write_decision(&mut w, decision);
    w.buf
}

/// Каноническое представление результата инспекции (CF-1…CF-17).
pub fn encode_inspection_result(result: &InspectionResult) -> Vec<u8> {
    let mut w = Writer::default();
    write_inspection_result(&mut w, result);
    w.buf
}

// ---------------------------------------------------------------------------------------------
// Запись JSON (CF-2, CF-9…CF-12)

#[derive(Default)]
struct Writer {
    buf: Vec<u8>,
}

impl Writer {
    fn raw(&mut self, s: &str) {
        self.buf.extend_from_slice(s.as_bytes());
    }

    /// Строка с экранированием CF-12: `"` и `\`, короткие формы для U+0008, U+0009, U+000A,
    /// U+000C, U+000D, `\u00xx` строчными цифрами для прочих U+0000…U+001F; остальное — байты
    /// UTF-8 как есть. Экранируемые символы — однобайтовые ASCII, поэтому обход по байтам не
    /// разрывает многобайтовые последовательности.
    fn string(&mut self, s: &str) {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        self.buf.push(b'"');
        for &b in s.as_bytes() {
            match b {
                b'"' => self.buf.extend_from_slice(b"\\\""),
                b'\\' => self.buf.extend_from_slice(b"\\\\"),
                0x08 => self.buf.extend_from_slice(b"\\b"),
                0x09 => self.buf.extend_from_slice(b"\\t"),
                0x0A => self.buf.extend_from_slice(b"\\n"),
                0x0C => self.buf.extend_from_slice(b"\\f"),
                0x0D => self.buf.extend_from_slice(b"\\r"),
                0x00..=0x1F => {
                    self.buf.extend_from_slice(b"\\u00");
                    self.buf.push(HEX[usize::from(b >> 4)]);
                    self.buf.push(HEX[usize::from(b & 0x0F)]);
                }
                _ => self.buf.push(b),
            }
        }
        self.buf.push(b'"');
    }

    fn uint(&mut self, n: u64) {
        self.raw(&n.to_string());
    }

    fn boolean(&mut self, b: bool) {
        self.raw(if b { "true" } else { "false" });
    }

    fn null(&mut self) {
        self.raw("null");
    }

    /// 64 строчные шестнадцатеричные цифры (CF-11).
    fn hex32(&mut self, bytes: &[u8; 32]) {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        self.buf.push(b'"');
        for &b in bytes {
            self.buf.push(HEX[usize::from(b >> 4)]);
            self.buf.push(HEX[usize::from(b & 0x0F)]);
        }
        self.buf.push(b'"');
    }

    fn object(&mut self, f: impl FnOnce(&mut Obj<'_>)) {
        self.buf.push(b'{');
        let mut o = Obj { w: self, first: true };
        f(&mut o);
        self.buf.push(b'}');
    }

    fn array<T>(&mut self, items: &[T], mut f: impl FnMut(&mut Writer, &T)) {
        self.buf.push(b'[');
        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                self.buf.push(b',');
            }
            f(self, item);
        }
        self.buf.push(b']');
    }
}

/// Объект JSON. Ключи передаются в порядке возрастания байтов (CF-3); порядок задан в коде
/// каждой функции записи и проверяется эталонами и проверяльщиком правил.
struct Obj<'a> {
    w: &'a mut Writer,
    first: bool,
}

impl Obj<'_> {
    fn key(&mut self, k: &str) -> &mut Writer {
        if !self.first {
            self.w.buf.push(b',');
        }
        self.first = false;
        self.w.string(k);
        self.w.buf.push(b':');
        self.w
    }

    fn str(&mut self, k: &str, v: &str) {
        self.key(k).string(v);
    }

    fn opt_str(&mut self, k: &str, v: Option<&str>) {
        let w = self.key(k);
        match v {
            Some(s) => w.string(s),
            None => w.null(),
        }
    }

    fn uint(&mut self, k: &str, v: u64) {
        self.key(k).uint(v);
    }

    fn opt_uint(&mut self, k: &str, v: Option<u64>) {
        let w = self.key(k);
        match v {
            Some(n) => w.uint(n),
            None => w.null(),
        }
    }

    fn boolean(&mut self, k: &str, v: bool) {
        self.key(k).boolean(v);
    }

    fn opt_enum<T: Copy>(&mut self, k: &str, v: Option<T>, name: fn(T) -> &'static str) {
        let w = self.key(k);
        match v {
            Some(x) => w.string(name(x)),
            None => w.null(),
        }
    }

    fn str_list<T>(&mut self, k: &str, items: &[T], s: fn(&T) -> &str) {
        self.key(k).array(items, |w, x| w.string(s(x)));
    }
}

// ---------------------------------------------------------------------------------------------
// Перечисления без полезной нагрузки (CF-6, таблица 5.7 контракта)

fn verdict(v: Verdict) -> &'static str {
    match v {
        Verdict::Allow => "ALLOW",
        Verdict::Block => "BLOCK",
        Verdict::Indeterminate => "INDETERMINATE",
    }
}

fn action(a: Action) -> &'static str {
    match a {
        Action::Monitor => "MONITOR",
        Action::Alert => "ALERT",
        Action::Block => "BLOCK",
    }
}

fn fallback(f: Fallback) -> &'static str {
    match f {
        Fallback::AllowWithAudit => "ALLOW_WITH_AUDIT",
        Fallback::Block => "BLOCK",
    }
}

fn decision_basis(b: DecisionBasis) -> &'static str {
    match b {
        DecisionBasis::RuleMatched => "RULE_MATCHED",
        DecisionBasis::NoRuleMatched => "NO_RULE_MATCHED",
        DecisionBasis::OutcomeIndependentOfUnknown => "OUTCOME_INDEPENDENT_OF_UNKNOWN",
        DecisionBasis::Indeterminate => "INDETERMINATE",
    }
}

fn action_certainty(c: ActionCertainty) -> &'static str {
    match c {
        ActionCertainty::Definite => "DEFINITE",
        ActionCertainty::Possible => "POSSIBLE",
    }
}

fn verification_mode(m: VerificationMode) -> &'static str {
    match m {
        VerificationMode::EnforcementReady => "ENFORCEMENT_READY",
        VerificationMode::InspectionOnly => "INSPECTION_ONLY",
    }
}

fn capability_check(c: CapabilityCheck) -> &'static str {
    match c {
        CapabilityCheck::Performed => "PERFORMED",
        CapabilityCheck::Skipped => "SKIPPED",
    }
}

fn data_level(d: DataLevel) -> &'static str {
    match d {
        DataLevel::Metadata => "METADATA",
        DataLevel::MaskedMatch => "MASKED_MATCH",
    }
}

fn deviation_input(d: DeviationInput) -> &'static str {
    match d {
        DeviationInput::Provided => "PROVIDED",
        DeviationInput::Unavailable => "UNAVAILABLE",
        DeviationInput::Contradictory => "CONTRADICTORY",
    }
}

fn truth(t: Truth) -> &'static str {
    match t {
        Truth::True => "TRUE",
        Truth::False => "FALSE",
        Truth::Unknown => "UNKNOWN",
    }
}

fn order_basis(b: OrderBasis) -> &'static str {
    match b {
        OrderBasis::Priority => "PRIORITY",
        OrderBasis::Strictness => "STRICTNESS",
        OrderBasis::RuleId => "RULE_ID",
    }
}

fn reason(r: ReasonCode) -> &'static str {
    r.as_str()
}

fn inspection_status(s: InspectionStatus) -> &'static str {
    match s {
        InspectionStatus::Match => "MATCH",
        InspectionStatus::NoMatch => "NO_MATCH",
        InspectionStatus::NotInspected => "NOT_INSPECTED",
    }
}

fn text_coverage(t: TextCoverage) -> &'static str {
    match t {
        TextCoverage::Complete => "COMPLETE",
        TextCoverage::Partial => "PARTIAL",
    }
}

fn ambiguity_reason(a: AmbiguityReason) -> &'static str {
    match a {
        AmbiguityReason::DeclaredContainerMismatch => "DECLARED_CONTAINER_MISMATCH",
        AmbiguityReason::EmbeddedSignature => "EMBEDDED_SIGNATURE",
    }
}

fn part_kind(k: PartKind) -> &'static str {
    match k {
        PartKind::Image => "IMAGE",
        PartKind::EmbeddedObject => "EMBEDDED_OBJECT",
        PartKind::Attachment => "ATTACHMENT",
        PartKind::Unknown => "UNKNOWN",
    }
}

fn inspection_origin(o: InspectionOrigin) -> &'static str {
    match o {
        InspectionOrigin::Core => "CORE",
        InspectionOrigin::HostSynthesized => "HOST_SYNTHESIZED",
    }
}

fn digest_status(s: DigestStatus) -> &'static str {
    match s {
        DigestStatus::Computed => "COMPUTED",
        DigestStatus::PartialRead => "PARTIAL_READ",
        DigestStatus::SizeLimit => "SIZE_LIMIT",
        DigestStatus::ReadError => "READ_ERROR",
        DigestStatus::NotRequested => "NOT_REQUESTED",
        DigestStatus::NoSnapshot => "NO_SNAPSHOT",
    }
}

fn digest_trust(t: DigestTrust) -> &'static str {
    match t {
        DigestTrust::TrustedSnapshot => "TRUSTED_SNAPSHOT",
        DigestTrust::Untrusted => "UNTRUSTED",
    }
}

// ---------------------------------------------------------------------------------------------
// Перечисления с полезной нагрузкой и кортежи (CF-7, CF-8; таблицы 5.3, 5.6 контракта)

fn write_structure_enumeration(w: &mut Writer, s: &StructureEnumeration) {
    w.object(|o| match s {
        StructureEnumeration::Complete => o.str("kind", "COMPLETE"),
        StructureEnumeration::Incomplete(r) => {
            o.str("kind", "INCOMPLETE");
            o.str("reason", reason(*r));
        }
    });
}

fn write_classification(w: &mut Writer, c: &Classification) {
    w.object(|o| match c {
        Classification::Confirmed => o.str("kind", "CONFIRMED"),
        Classification::Ambiguous(a) => {
            o.str("kind", "AMBIGUOUS");
            o.str("reason", ambiguity_reason(*a));
        }
    });
}

fn write_action_source(w: &mut Writer, s: &ActionSource) {
    w.object(|o| {
        o.str("action", action(s.action));
        let src = o.key("source");
        src.object(|so| match &s.source {
            ActionSourceRef::Rule(id) => {
                so.str("kind", "RULE");
                so.str("rule_id", id.as_str());
            }
            ActionSourceRef::IncompleteInspectionAction(id) => {
                so.str("kind", "INCOMPLETE_INSPECTION_ACTION");
                so.str("record_id", id.as_str());
            }
        });
    });
}

fn write_rule_pair(w: &mut Writer, pair: &(RuleId, RuleId)) {
    w.object(|o| {
        o.str("higher_rule_id", pair.0.as_str());
        o.str("lower_rule_id", pair.1.as_str());
    });
}

fn write_rule_trace(w: &mut Writer, t: &RuleTrace) {
    w.object(|o| {
        o.opt_str("determining_atom", t.determining_atom.as_deref());
        o.boolean("guards_incompleteness", t.guards_incompleteness);
        o.str("rule_id", t.rule_id.as_str());
        o.str("value", truth(t.value));
    });
}

// ---------------------------------------------------------------------------------------------
// Корневые и вложенные объекты (таблицы 5.1–5.5 контракта; ключи — по возрастанию байтов)

fn rule_id(r: &RuleId) -> &str {
    r.as_str()
}

fn write_decision(w: &mut Writer, d: &Decision) {
    w.object(|o| {
        o.opt_enum("action_certainty", d.action_certainty, action_certainty);
        o.key("action_sources").array(&d.action_sources, write_action_source);
        o.str("canonical_form", DECISION_CANONICAL_FORM);
        o.opt_enum("capability_check", d.capability_check, capability_check);
        o.opt_str("capability_exception_id", d.capability_exception_id.as_ref().map(|r| r.as_str()));
        o.str("data_level", data_level(d.data_level));
        o.str("decision_basis", decision_basis(d.decision_basis));
        o.boolean("degraded", d.degraded);
        o.str("deviation_input", deviation_input(d.deviation_input));
        write_explanation(o.key("explanation"), &d.explanation);
        o.opt_enum("fallback", d.fallback, fallback);
        o.str_list("inactive_deviation_ids", &d.inactive_deviation_ids, |r| r.as_str());
        o.opt_str("incomplete_action_source", d.incomplete_action_source.as_ref().map(|r| r.as_str()));
        o.boolean("inspection_complete", d.inspection_complete);
        o.str_list("matched_rules", &d.matched_rules, rule_id);
        o.opt_str("part_exclusion_id", d.part_exclusion_id.as_ref().map(|r| r.as_str()));
        o.opt_str("policy_bundle_id", d.policy_bundle_id.as_ref().map(|b| b.as_str()));
        o.opt_uint("policy_version", d.policy_version);
        o.str_list("possible_winners", &d.possible_winners, rule_id);
        o.str_list("precedence_approval_ids", &d.precedence_approval_ids, |r| r.as_str());
        o.key("reasons").array(&d.reasons, |w, r| w.string(reason(*r)));
        o.opt_enum("requested_action", d.requested_action, action);
        o.opt_enum("required_action", d.required_action, action);
        o.uint("schema_version", u64::from(d.schema_version));
        o.key("suppressed_precedence_pairs").array(&d.suppressed_precedence_pairs, write_rule_pair);
        o.str("verdict", verdict(d.verdict));
        o.opt_enum("verification_mode", d.verification_mode, verification_mode);
        o.opt_str("winner_rule_id", d.winner_rule_id.as_ref().map(|r| r.as_str()));
    });
}

fn write_explanation(w: &mut Writer, e: &Explanation) {
    w.object(|o| {
        o.opt_str("bundle_id", e.bundle_id.as_ref().map(|b| b.as_str()));
        o.opt_str("capability_exception_id", e.capability_exception_id.as_ref().map(|r| r.as_str()));
        o.key("class_worlds").array(&e.class_worlds, |w, c| {
            w.object(|co| match c {
                Some(fc) => {
                    co.str("file_class", fc.as_str());
                    co.str("kind", "CLASS");
                }
                None => co.str("kind", "OTHER"),
            })
        });
        {
            let cw = o.key("classification");
            match &e.classification {
                Some(c) => write_classification(cw, c),
                None => cw.null(),
            }
        }
        o.str("core_version", &e.core_version);
        o.str("detector_set_version", &e.detector_set_version);
        o.key("detectors").array(&e.detectors, |w, (id, count, met)| {
            w.object(|d| {
                d.str("detector_id", id.as_str());
                d.uint("match_count", u64::from(*count));
                d.boolean("threshold_met", *met);
            })
        });
        o.str_list("deviation_contradictions", &e.deviation_contradictions, |s| s.as_str());
        o.str_list("fallback_sources", &e.fallback_sources, rule_id);
        o.opt_str("file_class", e.file_class.as_ref().map(|c| c.as_str()));
        o.opt_str("incomplete_action_source", e.incomplete_action_source.as_ref().map(|r| r.as_str()));
        o.boolean("inspection_complete", e.inspection_complete);
        o.opt_str("part_exclusion_id", e.part_exclusion_id.as_ref().map(|r| r.as_str()));
        o.opt_enum("policy_state", e.policy_state, reason);
        o.opt_uint("policy_version", e.policy_version);
        o.str_list("possible_winners", &e.possible_winners, rule_id);
        o.str_list("precedence_approval_ids", &e.precedence_approval_ids, |r| r.as_str());
        o.key("rules").array(&e.rules, write_rule_trace);
        o.str_list("shadowed_incompleteness_guards", &e.shadowed_incompleteness_guards, rule_id);
        {
            let sw = o.key("structure_enumeration");
            match &e.structure_enumeration {
                Some(s) => write_structure_enumeration(sw, s),
                None => sw.null(),
            }
        }
        o.key("suppressed_precedence_pairs").array(&e.suppressed_precedence_pairs, write_rule_pair);
        o.opt_enum("text_coverage", e.text_coverage, text_coverage);
        o.str_list("unknown_rules", &e.unknown_rules, rule_id);
        o.opt_enum("winner_basis", e.winner_basis, order_basis);
        o.boolean("world_without_winner", e.world_without_winner);
        o.uint("worlds_considered", e.worlds_considered);
    });
}

fn write_inspection_result(w: &mut Writer, r: &InspectionResult) {
    w.object(|o| {
        o.str("canonical_form", INSPECTION_RESULT_CANONICAL_FORM);
        write_classification(o.key("classification"), &r.classification);
        o.key("detections").array(&r.detections, write_detection);
        write_digest(o.key("digest"), &r.digest);
        write_engine(o.key("engine"), &r.engine);
        o.str("file_class", r.file_class.as_str());
        o.str("inspection_origin", inspection_origin(r.inspection_origin));
        o.key("reasons").array(&r.reasons, |w, x| w.string(reason(*x)));
        o.uint("schema_version", u64::from(INSPECTION_RESULT_SCHEMA_VERSION));
        o.str("status", inspection_status(r.status));
        write_structure_enumeration(o.key("structure_enumeration"), &r.structure_enumeration);
        o.str("text_coverage", text_coverage(r.text_coverage));
        o.key("uninspected_parts").array(&r.uninspected_parts, write_uninspected_part);
    });
}

fn write_detection(w: &mut Writer, d: &DetectionResult) {
    w.object(|o| {
        o.str("detector_id", d.detector_id.as_str());
        o.str("detector_version", &d.detector_version);
        o.uint("match_count", u64::from(d.match_count));
        o.uint("score", u64::from(d.score));
        o.boolean("threshold_met", d.threshold_met);
    });
}

fn write_uninspected_part(w: &mut Writer, p: &UninspectedPart) {
    w.object(|o| {
        o.uint("count", u64::from(p.count));
        o.str("kind", part_kind(p.kind));
    });
}

fn write_digest(w: &mut Writer, d: &ContentDigest) {
    w.object(|o| {
        {
            let fw = o.key("full");
            match &d.full {
                Some(h) => fw.hex32(h),
                None => fw.null(),
            }
        }
        o.str("full_status", digest_status(d.full_status));
        o.key("read_set_digest").hex32(&d.read_set_digest);
        o.str("trust", digest_trust(d.trust));
    });
}

fn write_engine(w: &mut Writer, e: &EngineInfo) {
    w.object(|o| {
        o.str("core_version", &e.core_version);
        o.str("detector_set_version", &e.detector_set_version);
        o.str("limits_profile_id", &e.limits_profile_id);
    });
}
