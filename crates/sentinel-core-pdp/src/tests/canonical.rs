//! CHG-0008: каноническое представление на уровне PDP (матрица проверок, части D, F, L).
//!
//! - D-фикстуры строятся фабриками без общего состояния: каждый вызов заново строит IR
//!   политики, `CompiledPolicy`, контекст, `InspectionResult`, `DeviationValidity`, вызывает PDP
//!   и кодирует свежий результат (матрица, раздел 7.1). Рукописные эталоны D набраны вручную,
//!   независимо сверены ревьюером до сравнения и закреплены по SHA-256 (CHG8-IMPL-01);
//!   `d01_d12_match_handwritten_fixtures` сравнивает с ними вывод побайтно.
//! - L-01…L-05 — контрольные маркеры (матрица, раздел 6); пути полей определяются разбором вывода
//!   тестовым парсером JSON ниже. L-05a…L-05d активируют каждую заявленную запись бандла и
//!   сверяют точный набор путей каждого маркера (CHG8-IMPL-04).
//! - F-01: 100 повторов со свежими входами; F-02: 8 потоков, при заданной
//!   `SENTINEL_CANONICAL_THREADS_OUT` вывод каждого потока пишется в `thread-<k>/`.
//! - F-03: при заданной переменной окружения `SENTINEL_CANONICAL_OUT` тест записывает вывод
//!   D-фикстур в этот каталог; контрольные суммы вычисляются внешней утилитой.
//!
//! Подтверждаемая область — «canonical + PDP на фиксированных `InspectionResult`»; путь
//! «байты → инспекция → `InspectionResult`» не реализован и не проверяется.

use super::*;
use crate::policy::{Atom, IncompleteAction, Scope};
use crate::{PolicyState, PolicyStateKind};
use sentinel_core_types::canonical::encode_decision;

// ---------------------------------------------------------------- D-фикстуры (фабрики)

fn ambiguous_text(class: &str) -> InspectionResult {
    insp(
        InspectionStatus::NotInspected,
        &[ReasonCode::UnsupportedFormat],
        TextCoverage::Complete,
        StructureEnumeration::Complete,
        Classification::Ambiguous(AmbiguityReason::DeclaredContainerMismatch),
        &[],
        vec![det("card", 0, false)],
        class,
    )
}

fn sensitive_full() -> InspectionResult {
    insp(
        InspectionStatus::Match,
        &[],
        TextCoverage::Complete,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[],
        vec![det("sensitive", 2, true)],
        "TXT",
    )
}

fn policy_card_e1(with_e1: bool) -> CompiledPolicy<InspectionOnly> {
    let mut p = ir(vec![rule("Rc", 10, Action::Block, card(1))]);
    if with_e1 {
        p.exclusions.push(e1(None));
    }
    compiled(p)
}

fn policy_not_card_e1_expiring() -> CompiledPolicy<InspectionOnly> {
    let mut p = ir(vec![rule("Rn", 10, Action::Block, not(card(1)))]);
    p.exclusions.push(e1(Some(105)));
    compiled(p)
}

fn no_policy(kind: PolicyStateKind) -> Decision {
    let st = PolicyState { kind, bootstrap_fallback: Fallback::AllowWithAudit };
    crate::Pdp::decide_without_policy(&st, &ctx(), &not_inspected(ReasonCode::Timeout), &valid(&[]))
        .expect("решение без политики должно быть получено")
}

/// Идентификаторы D-фикстур (матрица, раздел 5).
const D_CASES: &[&str] = &[
    "D-01", "D-02", "D-03", "D-04", "D-05", "D-05b", "D-06", "D-07", "D-08", "D-09", "D-10", "D-11a",
    "D-11b", "D-11c", "D-12",
];

/// Строит входы заново и возвращает свежее решение PDP.
fn d_case(id: &str) -> Decision {
    match id {
        "D-01" => decide(
            &compiled(ir(vec![rule("R1", 10, Action::Block, card(1))])),
            &not_inspected(ReasonCode::ResourceExhausted),
            &valid(&[]),
        ),
        "D-02" => decide(
            &compiled(ir(vec![rule("R3", 10, Action::Block, incomplete(ReasonCode::ContentEncrypted))])),
            &not_inspected(ReasonCode::ContentEncrypted),
            &valid(&[]),
        ),
        "D-03" => decide(&policy_card_e1(true), &docx_with_image(0), &valid(&["E1"])),
        "D-04" => decide(
            &policy_card_e1(false),
            &insp(
                InspectionStatus::NoMatch,
                &[],
                TextCoverage::Complete,
                StructureEnumeration::Complete,
                Classification::Confirmed,
                &[],
                vec![det("card", 0, false)],
                "OOXML",
            ),
            &valid(&[]),
        ),
        "D-05" => {
            let mut p = ir(vec![
                rule("R9", 10, Action::Alert, usb()),
                rule("R10", 5, Action::Block, incomplete(ReasonCode::ContentEncrypted)),
            ]);
            p.approvals.push(pca("A1", "R9", "R10"));
            decide(&compiled(p), &not_inspected(ReasonCode::ContentEncrypted), &valid(&["A1"]))
        }
        "D-05b" => {
            let mut p = ir(vec![
                rule("R1", 100, Action::Alert, usb()),
                rule("R2", 10, Action::Block, count("sensitive", 1)),
            ]);
            p.approvals.push(pca("A1", "R1", "R2"));
            decide(&compiled(p), &sensitive_full(), &valid(&[]))
        }
        "D-06" => decide(
            &compiled(ir(vec![
                on_ind_block(rule("R16", 10, Action::Monitor, card(1))),
                rule("R15", 5, Action::Alert, usb()),
            ])),
            &not_inspected(ReasonCode::Timeout),
            &valid(&[]),
        ),
        "D-07" => {
            let mut p = ir(vec![rule("R30", 10, Action::Monitor, card(1))]);
            p.incomplete_actions.push(iia(
                "IIA-usb",
                Scope { channels: vec![Channel::new("USB")], ..Scope::default() },
                IncompleteAction::Alert,
            ));
            decide(&compiled(p), &not_inspected(ReasonCode::Timeout), &valid(&[]))
        }
        "D-08" => decide(
            &compiled(ir(vec![
                rule("R40", 10, Action::Alert, class_is("TXT")),
                rule("R41", 5, Action::Block, class_is("OOXML")),
            ])),
            &ambiguous_text("TXT"),
            &valid(&[]),
        ),
        "D-09" => decide(&policy_card_e1(true), &docx_with_image(0), &valid(&[])),
        "D-10" => decide(
            &policy_not_card_e1_expiring(),
            &docx_with_image(0),
            &DeviationValidity::Provided { time_state: TimeState::TimeUnverifiable, valid_ids: vec![] },
        ),
        "D-11a" => no_policy(PolicyStateKind::Absent),
        "D-11b" => no_policy(PolicyStateKind::Invalid),
        "D-11c" => no_policy(PolicyStateKind::Expired),
        "D-12" => decide(
            &policy_not_card_e1_expiring(),
            &docx_with_image(0),
            &DeviationValidity::Provided {
                time_state: TimeState::TimeUnverifiable,
                valid_ids: vec![rec("E1"), rec("HOST-UNKNOWN-7")],
            },
        ),
        other => panic!("неизвестная D-фикстура {other}"),
    }
}

fn d_bytes(id: &str) -> Vec<u8> {
    encode_decision(&d_case(id))
}

// ---------------------------------------------------------------- D-01…D-12: рукописные эталоны
//
// 15 эталонов набраны вручную (CHG8-IMPL-01), независимо сверены ревьюером до сравнения и
// закреплены по SHA-256 в `d-crosscheck-sealed.json` (2026-11-14 19:43:50 UTC). Суммы файлов
// сверяет сценарий проверок (D-SEALED); здесь — побайтовое сравнение вывода со свежими входами.
// При несовпадении тест падает: эталоны под вывод не подгоняются.

macro_rules! d_fixture {
    ($name:literal) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/canonical/", $name, ".json"))
            .as_slice()
    };
}

/// Идентификатор D-фикстуры → утверждённые байты.
const D_FIXTURES: &[(&str, &[u8])] = &[
    ("D-01", d_fixture!("D-01-indeterminate-unknown-rule")),
    ("D-02", d_fixture!("D-02-block-incompleteness-guard")),
    ("D-03", d_fixture!("D-03-exclusion-applied")),
    ("D-04", d_fixture!("D-04-no-rule-matched")),
    ("D-05", d_fixture!("D-05-approved-conflict-b")),
    ("D-05b", d_fixture!("D-05b-approval-inactive-pair-suppressed")),
    ("D-06", d_fixture!("D-06-two-allow-outcomes")),
    ("D-07", d_fixture!("D-07-contract-action-scoped-record")),
    ("D-08", d_fixture!("D-08-class-worlds-ambiguous")),
    ("D-09", d_fixture!("D-09-exclusion-not-valid")),
    ("D-10", d_fixture!("D-10-time-unverifiable-empty-valid-ids")),
    ("D-11a", d_fixture!("D-11a-no-policy-absent")),
    ("D-11b", d_fixture!("D-11b-no-policy-invalid")),
    ("D-11c", d_fixture!("D-11c-no-policy-expired")),
    ("D-12", d_fixture!("D-12-contradictory-time-and-unknown-id")),
];

/// Первое отличающееся смещение и окрестность — для диагностики без вывода всего документа.
fn first_difference(got: &[u8], want: &[u8]) -> String {
    let i = got.iter().zip(want).position(|(a, b)| a != b).unwrap_or(got.len().min(want.len()));
    let w = |b: &[u8]| String::from_utf8_lossy(&b[i.saturating_sub(40)..(i + 40).min(b.len())]).into_owned();
    format!(
        "длина {} против {}; первое отличие на байте {i}: …{}… против …{}…",
        got.len(),
        want.len(),
        w(got),
        w(want)
    )
}

/// Состав D-фикстур совпадает с `D_CASES`, без повторов.
#[test]
fn d00_fixture_set_matches_cases() {
    let ids: Vec<&str> = D_FIXTURES.iter().map(|(id, _)| *id).collect();
    assert_eq!(ids, D_CASES, "эталоны D не соответствуют списку случаев");
}

/// D-01…D-12, D-05b, D-11a/b/c: свежие входы → PDP → кодирование → побайтно равно эталону.
#[test]
fn d01_d12_match_handwritten_fixtures() {
    let mut bad = vec![];
    for (id, want) in D_FIXTURES {
        let got = d_bytes(id);
        if got.as_slice() != *want {
            bad.push(format!("{id}: {}", first_difference(&got, want)));
        }
    }
    assert!(bad.is_empty(), "вывод не совпал с рукописными эталонами:\n{}", bad.join("\n"));
}

// ---------------------------------------------------------------- F-01, F-02, F-03 (D-фикстуры)

#[test]
fn f01_d_cases_repeat_in_process() {
    for id in D_CASES {
        let first = d_bytes(id);
        for _ in 0..100 {
            assert_eq!(d_bytes(id), first, "{id}: повтор со свежими входами дал другие байты");
        }
    }
}

/// Запись вывода D-фикстур `<каталог>/<ID>.json`; суммы вычисляет внешняя утилита.
fn write_d_outputs(dir: &std::path::Path, outputs: &[Vec<u8>]) {
    std::fs::create_dir_all(dir).expect("каталог вывода");
    for (id, bytes) in D_CASES.iter().zip(outputs) {
        std::fs::write(dir.join(format!("{id}.json")), bytes).expect("запись вывода");
    }
}

/// F-02 (D): 1 поток против 8 потоков; каждый поток сам строит входы и вызывает PDP. При заданной
/// `SENTINEL_CANONICAL_THREADS_OUT` вывод пишется в `single/` и `thread-<k>/` этого каталога.
#[test]
fn f02_d_cases_threads() {
    let single: Vec<Vec<u8>> = D_CASES.iter().map(|id| d_bytes(id)).collect();
    let per_thread: Vec<Vec<Vec<u8>>> = std::thread::scope(|s| {
        let handles: Vec<_> =
            (0..8).map(|_| s.spawn(|| D_CASES.iter().map(|id| d_bytes(id)).collect::<Vec<_>>())).collect();
        handles.into_iter().map(|h| h.join().expect("поток проверки завершился паникой")).collect()
    });
    if let Some(dir) = std::env::var_os("SENTINEL_CANONICAL_THREADS_OUT") {
        let dir = std::path::PathBuf::from(dir);
        write_d_outputs(&dir.join("single"), &single);
        for (k, outputs) in per_thread.iter().enumerate() {
            write_d_outputs(&dir.join(format!("thread-{k}")), outputs);
        }
    }
    for (k, outputs) in per_thread.iter().enumerate() {
        assert_eq!(outputs, &single, "поток {k}: байты отличаются от однопоточного прогона");
    }
}

/// F-03, F-05: запись вывода D-фикстур в каталог `SENTINEL_CANONICAL_OUT` (если задан).
/// Средство контрольных сумм не вычисляет: суммы считает внешняя утилита по записанным файлам.
#[test]
fn f03_emit_d_outputs() {
    let Some(dir) = std::env::var_os("SENTINEL_CANONICAL_OUT") else { return };
    let outputs: Vec<Vec<u8>> = D_CASES.iter().map(|id| d_bytes(id)).collect();
    write_d_outputs(std::path::Path::new(&dir), &outputs);
}

// ---------------------------------------------------------------- тестовый разбор JSON (для L)

#[derive(Debug)]
enum Json {
    Null,
    Bool,
    Num,
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

struct P<'a> {
    b: &'a [u8],
    i: usize,
}

impl P<'_> {
    fn peek(&self) -> u8 {
        self.b.get(self.i).copied().unwrap_or(0)
    }
    fn expect(&mut self, c: u8) {
        assert_eq!(self.peek(), c, "JSON: ожидался {:?} на {}", c as char, self.i);
        self.i += 1;
    }
    fn value(&mut self) -> Json {
        match self.peek() {
            b'n' => {
                self.i += 4;
                Json::Null
            }
            b't' => {
                self.i += 4;
                Json::Bool
            }
            b'f' => {
                self.i += 5;
                Json::Bool
            }
            b'"' => Json::Str(self.string()),
            b'[' => {
                self.i += 1;
                let mut v = vec![];
                if self.peek() == b']' {
                    self.i += 1;
                    return Json::Arr(v);
                }
                loop {
                    v.push(self.value());
                    if self.peek() == b',' {
                        self.i += 1;
                    } else {
                        self.expect(b']');
                        return Json::Arr(v);
                    }
                }
            }
            b'{' => {
                self.i += 1;
                let mut v = vec![];
                if self.peek() == b'}' {
                    self.i += 1;
                    return Json::Obj(v);
                }
                loop {
                    let k = self.string();
                    self.expect(b':');
                    v.push((k, self.value()));
                    if self.peek() == b',' {
                        self.i += 1;
                    } else {
                        self.expect(b'}');
                        return Json::Obj(v);
                    }
                }
            }
            _ => {
                while self.peek().is_ascii_digit() {
                    self.i += 1;
                }
                Json::Num
            }
        }
    }
    fn string(&mut self) -> String {
        self.expect(b'"');
        let mut out = Vec::new();
        loop {
            let c = self.peek();
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = self.peek();
                    self.i += 1;
                    match e {
                        b'u' => {
                            let hex = std::str::from_utf8(&self.b[self.i..self.i + 4]).expect("hex");
                            let cp = u32::from_str_radix(hex, 16).expect("hex");
                            self.i += 4;
                            let ch = char::from_u32(cp).expect("код");
                            out.extend_from_slice(ch.to_string().as_bytes());
                        }
                        b'b' => out.push(0x08),
                        b't' => out.push(b'\t'),
                        b'n' => out.push(b'\n'),
                        b'f' => out.push(0x0C),
                        b'r' => out.push(b'\r'),
                        other => out.push(other),
                    }
                }
                other => out.push(other),
            }
        }
        String::from_utf8(out).expect("UTF-8")
    }
}

fn parse(bytes: &[u8]) -> Json {
    let mut p = P { b: bytes, i: 0 };
    let v = p.value();
    assert_eq!(p.i, bytes.len(), "JSON: лишние байты");
    v
}

/// Все строковые значения с путями вида `explanation.rules[].rule_id` (индексы опущены).
fn string_leaves(v: &Json, path: &str, out: &mut Vec<(String, String)>) {
    match v {
        Json::Str(s) => out.push((path.to_string(), s.clone())),
        Json::Arr(items) => {
            for x in items {
                string_leaves(x, &format!("{path}[]"), out);
            }
        }
        Json::Obj(fields) => {
            for (k, x) in fields {
                let p = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                string_leaves(x, &p, out);
            }
        }
        Json::Null | Json::Bool | Json::Num => {}
    }
}

fn keys(v: &Json, out: &mut Vec<String>) {
    match v {
        Json::Arr(items) => items.iter().for_each(|x| keys(x, out)),
        Json::Obj(fields) => {
            for (k, x) in fields {
                out.push(k.clone());
                keys(x, out);
            }
        }
        _ => {}
    }
}

/// Пути, в которых встречается маркер (подстрокой значения).
fn marker_paths(bytes: &[u8], marker: &str) -> Vec<String> {
    let mut leaves = vec![];
    string_leaves(&parse(bytes), "", &mut leaves);
    leaves.into_iter().filter(|(_, s)| s.contains(marker)).map(|(p, _)| p).collect()
}

fn occurrences(hay: &[u8], needle: &str) -> usize {
    let n = needle.as_bytes();
    hay.windows(n.len()).filter(|w| *w == n).count()
}

// ---------------------------------------------------------------- L-01: точное сохранение

#[test]
fn l01_every_string_slot_is_preserved_exactly_once_at_its_path() {
    let m = |p: &str| format!("MK-L01-{p}");
    let d = Decision {
        verdict: Verdict::Allow,
        reasons: vec![],
        policy_version: Some(1),
        policy_bundle_id: Some(BundleId::new(m("policy_bundle_id"))),
        matched_rules: vec![rid(&m("matched_rules"))],
        required_action: Some(Action::Alert),
        decision_basis: DecisionBasis::RuleMatched,
        inspection_complete: true,
        winner_rule_id: Some(rid(&m("winner_rule_id"))),
        possible_winners: vec![rid(&m("possible_winners"))],
        part_exclusion_id: Some(rec(&m("part_exclusion_id"))),
        precedence_approval_ids: vec![rec(&m("precedence_approval_ids"))],
        suppressed_precedence_pairs: vec![(rid(&m("pair_higher")), rid(&m("pair_lower")))],
        inactive_deviation_ids: vec![rec(&m("inactive_deviation_ids"))],
        deviation_input: DeviationInput::Provided,
        incomplete_action_source: Some(rec(&m("incomplete_action_source"))),
        action_certainty: Some(ActionCertainty::Definite),
        action_sources: vec![
            ActionSource { source: ActionSourceRef::Rule(rid(&m("src_rule"))), action: Action::Alert },
            ActionSource {
                source: ActionSourceRef::IncompleteInspectionAction(rec(&m("src_iia"))),
                action: Action::Monitor,
            },
        ],
        verification_mode: Some(VerificationMode::InspectionOnly),
        capability_check: Some(CapabilityCheck::Skipped),
        requested_action: None,
        capability_exception_id: Some(rec(&m("capability_exception_id"))),
        fallback: None,
        data_level: DataLevel::Metadata,
        degraded: false,
        explanation: Explanation {
            core_version: m("e_core_version"),
            detector_set_version: m("e_detector_set_version"),
            bundle_id: Some(BundleId::new(m("e_bundle_id"))),
            policy_version: Some(1),
            file_class: Some(FileClass::new(m("e_file_class"))),
            classification: Some(Classification::Confirmed),
            text_coverage: Some(TextCoverage::Complete),
            structure_enumeration: Some(StructureEnumeration::Complete),
            inspection_complete: true,
            detectors: vec![(DetectorId::new(m("e_detector")), 1, true)],
            rules: vec![RuleTrace {
                rule_id: rid(&m("e_rule")),
                value: Truth::True,
                determining_atom: Some(m("e_atom")),
                guards_incompleteness: false,
            }],
            winner_basis: None,
            unknown_rules: vec![rid(&m("e_unknown"))],
            possible_winners: vec![rid(&m("e_possible"))],
            world_without_winner: false,
            worlds_considered: 1,
            class_worlds: vec![Some(FileClass::new(m("e_class_world")))],
            fallback_sources: vec![rid(&m("e_fallback"))],
            part_exclusion_id: Some(rec(&m("e_part_exclusion"))),
            precedence_approval_ids: vec![rec(&m("e_approval"))],
            suppressed_precedence_pairs: vec![(rid(&m("e_pair_h")), rid(&m("e_pair_l")))],
            incomplete_action_source: Some(rec(&m("e_iia"))),
            capability_exception_id: Some(rec(&m("e_cx"))),
            shadowed_incompleteness_guards: vec![rid(&m("e_shadowed"))],
            deviation_contradictions: vec![m("e_contradiction")],
            policy_state: None,
        },
        schema_version: 1,
    };
    let expected: &[(&str, &str)] = &[
        ("policy_bundle_id", "policy_bundle_id"),
        ("matched_rules", "matched_rules[]"),
        ("winner_rule_id", "winner_rule_id"),
        ("possible_winners", "possible_winners[]"),
        ("part_exclusion_id", "part_exclusion_id"),
        ("precedence_approval_ids", "precedence_approval_ids[]"),
        ("pair_higher", "suppressed_precedence_pairs[].higher_rule_id"),
        ("pair_lower", "suppressed_precedence_pairs[].lower_rule_id"),
        ("inactive_deviation_ids", "inactive_deviation_ids[]"),
        ("incomplete_action_source", "incomplete_action_source"),
        ("src_rule", "action_sources[].source.rule_id"),
        ("src_iia", "action_sources[].source.record_id"),
        ("capability_exception_id", "capability_exception_id"),
        ("e_core_version", "explanation.core_version"),
        ("e_detector_set_version", "explanation.detector_set_version"),
        ("e_bundle_id", "explanation.bundle_id"),
        ("e_file_class", "explanation.file_class"),
        ("e_detector", "explanation.detectors[].detector_id"),
        ("e_rule", "explanation.rules[].rule_id"),
        ("e_atom", "explanation.rules[].determining_atom"),
        ("e_unknown", "explanation.unknown_rules[]"),
        ("e_possible", "explanation.possible_winners[]"),
        ("e_class_world", "explanation.class_worlds[].file_class"),
        ("e_fallback", "explanation.fallback_sources[]"),
        ("e_part_exclusion", "explanation.part_exclusion_id"),
        ("e_approval", "explanation.precedence_approval_ids[]"),
        ("e_pair_h", "explanation.suppressed_precedence_pairs[].higher_rule_id"),
        ("e_pair_l", "explanation.suppressed_precedence_pairs[].lower_rule_id"),
        ("e_iia", "explanation.incomplete_action_source"),
        ("e_cx", "explanation.capability_exception_id"),
        ("e_shadowed", "explanation.shadowed_incompleteness_guards[]"),
        ("e_contradiction", "explanation.deviation_contradictions[]"),
    ];
    let out = encode_decision(&d);
    for (suffix, path) in expected {
        // Маркер с завершающей кавычкой: `MK-L01-e_pair_h` не должен совпасть внутри `…e_pair_hX`.
        let marker = m(suffix);
        assert_eq!(
            occurrences(&out, &format!("\"{marker}\"")),
            1,
            "{marker}: ожидалось ровно одно вхождение"
        );
        let paths: Vec<String> = marker_paths(&out, &marker).into_iter().filter(|p| p == path).collect();
        assert_eq!(paths, vec![path.to_string()], "{marker}");
    }
    let mut all_keys = vec![];
    keys(&parse(&out), &mut all_keys);
    assert!(all_keys.iter().all(|k| !k.contains("MK-")), "маркер в ключе");
}

// ---------------------------------------------------------------- L-02…L-05: PDP и источники строк

/// Контекст с маркерами субъекта, устройства, групп, подразделений, канала и операции.
fn marked_ctx() -> DecisionContext {
    DecisionContext {
        channel: Channel::new("MK-CTX-CHANNEL"),
        operation: Operation::new("MK-CTX-OPERATION"),
        principal_ref: PrincipalRef::new("MK-CTX-PRINCIPAL"),
        device_ref: DeviceRef::new("MK-CTX-DEVICE"),
        scope_attrs: vec![
            ScopeAttr::Group(GroupId::new("MK-CTX-GROUP")),
            ScopeAttr::OrgUnit(OrgUnitId::new("MK-CTX-ORGUNIT")),
        ],
        policy_validity: PolicyValidity::Current,
        decision_time: None,
    }
}

/// Политика с маркерами в идентификаторах; области правил содержат значения контекста.
fn marked_policy() -> CompiledPolicy<InspectionOnly> {
    let scope = Scope {
        channels: vec![Channel::new("MK-CTX-CHANNEL")],
        operations: vec![Operation::new("MK-CTX-OPERATION")],
        groups: vec![GroupId::new("MK-CTX-GROUP")],
        org_units: vec![OrgUnitId::new("MK-CTX-ORGUNIT")],
        devices: vec![DeviceRef::new("MK-CTX-DEVICE")],
    };
    let mut r1 = rule(
        "MK-POL-RULE-HIGH",
        20,
        Action::Alert,
        Expr::Atom(Atom::ChannelIs(Channel::new("MK-POL-CHANNEL"))),
    );
    r1.scope = scope.clone();
    let mut r2 = rule(
        "MK-POL-RULE-LOW",
        10,
        Action::Monitor,
        Expr::Atom(Atom::OperationIs(Operation::new("MK-POL-OPERATION"))),
    );
    r2.scope = scope.clone();
    let mut r3 = rule("MK-POL-RULE-DET", 5, Action::Alert, count("MK-POL-DETECTOR", 1));
    r3.scope = scope;
    let mut p = ir(vec![r1, r2, r3]);
    p.bundle_id = BundleId::new("MK-POL-BUNDLE");
    p.detector_set
        .push(crate::policy::DetectorSpec { detector_id: DetectorId::new("MK-POL-DETECTOR"), threshold: 1 });
    p.incomplete_actions = vec![iia("MK-POL-IIA", Scope::default(), IncompleteAction::Monitor)];
    compiled(p)
}

fn marked_inspection() -> InspectionResult {
    insp(
        InspectionStatus::Match,
        &[],
        TextCoverage::Complete,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[],
        vec![det("MK-POL-DETECTOR", 3, true)],
        "TXT",
    )
}

fn marked_decision(dv: &DeviationValidity) -> Vec<u8> {
    let d = crate::Pdp::decide(&marked_policy(), &marked_ctx(), &marked_inspection(), dv)
        .expect("решение должно быть получено");
    encode_decision(&d)
}

#[test]
fn l02_identity_and_scope_values_do_not_appear() {
    let out = marked_decision(&valid(&[]));
    for m in ["MK-CTX-PRINCIPAL", "MK-CTX-DEVICE", "MK-CTX-GROUP", "MK-CTX-ORGUNIT"] {
        assert_eq!(occurrences(&out, m), 0, "{m} не должен попадать в каноническое решение");
    }
    // Правило с областью применилось — проверка не вырожденная.
    assert!(occurrences(&out, "MK-POL-RULE-DET") > 0);
}

#[test]
fn l03_context_channel_and_operation_values_do_not_appear() {
    let out = marked_decision(&valid(&[]));
    assert_eq!(occurrences(&out, "MK-CTX-CHANNEL"), 0);
    assert_eq!(occurrences(&out, "MK-CTX-OPERATION"), 0);
    for m in ["MK-POL-CHANNEL", "MK-POL-OPERATION"] {
        let paths = marker_paths(&out, m);
        assert!(!paths.is_empty(), "{m}: константа атома должна присутствовать в объяснении");
        assert!(paths.iter().all(|p| p == "explanation.rules[].determining_atom"), "{m}: {paths:?}");
    }
}

#[test]
fn l04_host_deviation_ids_only_in_contradictions() {
    let dv = valid(&["MK-HOST-DEVIATION"]);
    let out = marked_decision(&dv);
    let paths = marker_paths(&out, "MK-HOST-DEVIATION");
    assert!(!paths.is_empty(), "идентификатор хоста вне бандла должен быть отражён как противоречие");
    assert!(paths.iter().all(|p| p == "explanation.deviation_contradictions[]"), "{paths:?}");
    let s = String::from_utf8(out).unwrap_or_default();
    assert!(s.contains(r#""deviation_input":"CONTRADICTORY""#), "{s}");
}

// ---------------------------------------------------------------- L-05: строки бандла (CHG8-IMPL-04)
//
// Каждый сценарий активирует свои записи бандла и задаёт для каждого маркера точный набор
// путей. Проверка: (1) набор путей маркера равен ожидаемому — пустой набор означает «маркер
// не должен появиться»; (2) все маркеры `MK-POL-*` в выводе объявлены сценарием; (3) каждый путь
// с маркером входит в поля источников S1/S3 (контракт, раздел 10.2). Общего счётчика нет:
// отсутствие объявленного маркера — FAIL. Маркеры не являются префиксами друг друга.

/// Поля источников S1 и S3 таблицы раздела 10.2 контракта, которые могут нести строки бандла.
const L05_ALLOWED_S1_S3: &[&str] = &[
    "matched_rules[]",
    "winner_rule_id",
    "possible_winners[]",
    "part_exclusion_id",
    "precedence_approval_ids[]",
    "suppressed_precedence_pairs[].higher_rule_id",
    "suppressed_precedence_pairs[].lower_rule_id",
    "inactive_deviation_ids[]",
    "incomplete_action_source",
    "action_sources[].source.rule_id",
    "action_sources[].source.record_id",
    "capability_exception_id",
    "policy_bundle_id",
    "explanation.bundle_id",
    "explanation.rules[].rule_id",
    "explanation.rules[].determining_atom",
    "explanation.unknown_rules[]",
    "explanation.possible_winners[]",
    "explanation.fallback_sources[]",
    "explanation.part_exclusion_id",
    "explanation.precedence_approval_ids[]",
    "explanation.suppressed_precedence_pairs[].higher_rule_id",
    "explanation.suppressed_precedence_pairs[].lower_rule_id",
    "explanation.incomplete_action_source",
    "explanation.capability_exception_id",
    "explanation.shadowed_incompleteness_guards[]",
    "explanation.class_worlds[].file_class",
    "explanation.detectors[].detector_id",
];

/// Маркеры `MK-POL-…` в строке: максимальные последовательности `[A-Z0-9-]` после префикса.
fn pol_tokens(s: &str) -> Vec<String> {
    let mut out = vec![];
    let mut rest = s;
    while let Some(i) = rest.find("MK-POL-") {
        let tail = &rest[i..];
        let end = tail
            .char_indices()
            .find(|(_, c)| !(c.is_ascii_uppercase() || c.is_ascii_digit() || *c == '-'))
            .map_or(tail.len(), |(k, _)| k);
        out.push(tail[..end].to_string());
        rest = &tail[end..];
    }
    out
}

/// Проверка сценария L-05: `expected` — маркер и точный набор путей (пустой — маркера нет).
fn l05_check(case: &str, out: &[u8], expected: &[(&str, &[&str])]) {
    let mut leaves = vec![];
    string_leaves(&parse(out), "", &mut leaves);
    let mut found: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> =
        std::collections::BTreeMap::new();
    for (path, value) in &leaves {
        assert!(!value.contains("MK-CTX-"), "{case}: маркер контекста в {path}: {value}");
        for t in pol_tokens(value) {
            assert!(
                L05_ALLOWED_S1_S3.contains(&path.as_str()),
                "{case}: маркер {t} в пути вне S1/S3: {path}"
            );
            found.entry(t).or_default().insert(path.clone());
        }
    }
    let declared: std::collections::BTreeSet<&str> = expected.iter().map(|(m, _)| *m).collect();
    for m in found.keys() {
        assert!(declared.contains(m.as_str()), "{case}: необъявленный маркер {m}: {:?}", found[m]);
    }
    for (m, paths) in expected {
        let want: std::collections::BTreeSet<String> = paths.iter().map(|p| p.to_string()).collect();
        let got = found.get(*m).cloned().unwrap_or_default();
        assert_eq!(got, want, "{case}: пути маркера {m}");
    }
}

fn l05_detectors() -> Vec<crate::policy::DetectorSpec> {
    vec![crate::policy::DetectorSpec { detector_id: DetectorId::new("MK-POL-DETECTOR"), threshold: 1 }]
}

fn l05_ir(rules: Vec<crate::policy::Rule>) -> crate::policy::PolicyIr {
    let mut p = ir(rules);
    p.bundle_id = BundleId::new("MK-POL-BUNDLE");
    p.detector_set = l05_detectors();
    p.incomplete_actions = vec![iia("MK-POL-IIA", Scope::default(), IncompleteAction::Monitor)];
    p
}

fn l05_incomplete_insp(class: &str, classification: Classification) -> InspectionResult {
    insp(
        InspectionStatus::NotInspected,
        &[ReasonCode::Timeout],
        TextCoverage::Partial,
        StructureEnumeration::Complete,
        classification,
        &[],
        vec![det("MK-POL-DETECTOR", 0, false)],
        class,
    )
}

/// L-05a: ALLOW с записью контракта неполной инспекции — `MK-POL-IIA` влияет на решение
/// (`!complete && world_without_winner`, decide.rs, шаг 7).
#[test]
fn l05a_incomplete_inspection_action_record() {
    let p = compiled(l05_ir(vec![rule("MK-POL-RULE-DET", 10, Action::Alert, count("MK-POL-DETECTOR", 1))]));
    let d =
        crate::Pdp::decide(&p, &ctx(), &l05_incomplete_insp("TXT", Classification::Confirmed), &valid(&[]))
            .expect("решение");
    assert_eq!(d.incomplete_action_source, Some(rec("MK-POL-IIA")), "сценарий не активировал запись");
    l05_check(
        "L-05a",
        &encode_decision(&d),
        &[
            (
                "MK-POL-IIA",
                &[
                    "incomplete_action_source",
                    "action_sources[].source.record_id",
                    "explanation.incomplete_action_source",
                ],
            ),
            (
                "MK-POL-RULE-DET",
                &[
                    "possible_winners[]",
                    "action_sources[].source.rule_id",
                    "explanation.rules[].rule_id",
                    "explanation.unknown_rules[]",
                    "explanation.possible_winners[]",
                ],
            ),
            (
                "MK-POL-DETECTOR",
                &["explanation.detectors[].detector_id", "explanation.rules[].determining_atom"],
            ),
            ("MK-POL-BUNDLE", &["policy_bundle_id", "explanation.bundle_id"]),
        ],
    );
}

/// L-05b: применённое исключение, действительное утверждение конфликта, недействительное
/// исключение, исключение возможностей победителя и затенённое правило с атомом неполноты.
#[test]
fn l05b_exclusion_approval_capability_and_inactive_records() {
    let mut high = rule("MK-POL-RULE-HIGH", 20, Action::Alert, usb());
    high.requested_action = Some(Action::Block);
    high.capability_exception_id = Some(rec("MK-POL-CAPEX"));
    let low = rule("MK-POL-RULE-LOW", 10, Action::Block, count("MK-POL-DETECTOR", 1));
    let guard = rule("MK-POL-RULE-GUARD", 5, Action::Monitor, Expr::Atom(Atom::InspectionIncomplete(None)));
    let mut p = l05_ir(vec![high, low, guard]);
    let excl = |id: &str| crate::policy::PartInspectionExclusion { exclusion_id: rec(id), ..e1(None) };
    p.exclusions = vec![excl("MK-POL-EXCL-ACTIVE"), excl("MK-POL-EXCL-INACTIVE")];
    p.approvals = vec![pca("MK-POL-APPR-ACTIVE", "MK-POL-RULE-HIGH", "MK-POL-RULE-LOW")];
    let i = insp(
        InspectionStatus::NotInspected,
        &[ReasonCode::UnsupportedFormat],
        TextCoverage::Complete,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[PartKind::Image],
        vec![det("MK-POL-DETECTOR", 0, false)],
        "OOXML",
    );
    let d =
        crate::Pdp::decide(&compiled(p), &ctx(), &i, &valid(&["MK-POL-EXCL-ACTIVE", "MK-POL-APPR-ACTIVE"]))
            .expect("решение");
    assert_eq!(d.part_exclusion_id, Some(rec("MK-POL-EXCL-ACTIVE")), "исключение не применено");
    assert_eq!(d.precedence_approval_ids, vec![rec("MK-POL-APPR-ACTIVE")], "утверждение не использовано");
    assert_eq!(d.capability_exception_id, Some(rec("MK-POL-CAPEX")), "победитель не перенёс исключение");
    l05_check(
        "L-05b",
        &encode_decision(&d),
        &[
            (
                "MK-POL-RULE-HIGH",
                &[
                    "winner_rule_id",
                    "matched_rules[]",
                    "action_sources[].source.rule_id",
                    "explanation.rules[].rule_id",
                    "explanation.possible_winners[]",
                ],
            ),
            ("MK-POL-RULE-LOW", &["explanation.rules[].rule_id"]),
            (
                "MK-POL-RULE-GUARD",
                &[
                    "matched_rules[]",
                    "explanation.rules[].rule_id",
                    "explanation.shadowed_incompleteness_guards[]",
                ],
            ),
            ("MK-POL-EXCL-ACTIVE", &["part_exclusion_id", "explanation.part_exclusion_id"]),
            ("MK-POL-EXCL-INACTIVE", &["inactive_deviation_ids[]"]),
            ("MK-POL-APPR-ACTIVE", &["precedence_approval_ids[]", "explanation.precedence_approval_ids[]"]),
            ("MK-POL-CAPEX", &["capability_exception_id", "explanation.capability_exception_id"]),
            (
                "MK-POL-DETECTOR",
                &["explanation.detectors[].detector_id", "explanation.rules[].determining_atom"],
            ),
            ("MK-POL-BUNDLE", &["policy_bundle_id", "explanation.bundle_id"]),
            ("MK-POL-IIA", &[]),
        ],
    );
}

/// L-05c: недействительное утверждение — пара подавлена, INDETERMINATE с источниками fallback.
#[test]
fn l05c_suppressed_pair_and_fallback_sources() {
    let mut p = l05_ir(vec![
        rule("MK-POL-RULE-SUPH", 20, Action::Alert, usb()),
        rule("MK-POL-RULE-SUPL", 10, Action::Block, count("MK-POL-DETECTOR", 1)),
    ]);
    p.approvals = vec![pca("MK-POL-APPR-INACTIVE", "MK-POL-RULE-SUPH", "MK-POL-RULE-SUPL")];
    let d = crate::Pdp::decide(
        &compiled(p),
        &ctx(),
        &l05_incomplete_insp("TXT", Classification::Confirmed),
        &valid(&[]),
    )
    .expect("решение");
    assert_eq!(d.verdict, Verdict::Indeterminate, "сценарий не дал INDETERMINATE");
    assert_eq!(d.suppressed_precedence_pairs.len(), 1, "пара не подавлена");
    l05_check(
        "L-05c",
        &encode_decision(&d),
        &[
            (
                "MK-POL-RULE-SUPH",
                &[
                    "matched_rules[]",
                    "possible_winners[]",
                    "suppressed_precedence_pairs[].higher_rule_id",
                    "explanation.rules[].rule_id",
                    "explanation.possible_winners[]",
                    "explanation.fallback_sources[]",
                    "explanation.suppressed_precedence_pairs[].higher_rule_id",
                ],
            ),
            (
                "MK-POL-RULE-SUPL",
                &[
                    "possible_winners[]",
                    "suppressed_precedence_pairs[].lower_rule_id",
                    "explanation.rules[].rule_id",
                    "explanation.possible_winners[]",
                    "explanation.fallback_sources[]",
                    "explanation.suppressed_precedence_pairs[].lower_rule_id",
                ],
            ),
            ("MK-POL-APPR-INACTIVE", &["inactive_deviation_ids[]"]),
            (
                "MK-POL-DETECTOR",
                &["explanation.detectors[].detector_id", "explanation.rules[].determining_atom"],
            ),
            ("MK-POL-BUNDLE", &["policy_bundle_id", "explanation.bundle_id"]),
            ("MK-POL-IIA", &[]),
        ],
    );
}

/// L-05d: неоднозначная классификация — константа `file.class` бандла в мирах класса.
#[test]
fn l05d_class_worlds_constant() {
    let p = compiled(l05_ir(vec![rule("MK-POL-RULE-CLS", 10, Action::Alert, class_is("MK-POL-CLASS-A"))]));
    let i = l05_incomplete_insp("TXT", Classification::Ambiguous(AmbiguityReason::DeclaredContainerMismatch));
    let d = crate::Pdp::decide(&p, &ctx(), &i, &valid(&[])).expect("решение");
    assert_eq!(d.explanation.class_worlds.len(), 2, "миры класса не построены");
    l05_check(
        "L-05d",
        &encode_decision(&d),
        &[
            (
                "MK-POL-CLASS-A",
                &["explanation.class_worlds[].file_class", "explanation.rules[].determining_atom"],
            ),
            (
                "MK-POL-RULE-CLS",
                &[
                    "possible_winners[]",
                    "action_sources[].source.rule_id",
                    "explanation.rules[].rule_id",
                    "explanation.unknown_rules[]",
                    "explanation.possible_winners[]",
                ],
            ),
            (
                "MK-POL-IIA",
                &[
                    "incomplete_action_source",
                    "action_sources[].source.record_id",
                    "explanation.incomplete_action_source",
                ],
            ),
            ("MK-POL-DETECTOR", &["explanation.detectors[].detector_id"]),
            ("MK-POL-BUNDLE", &["policy_bundle_id", "explanation.bundle_id"]),
        ],
    );
}

/// Самопроверка L-05: оценщик не проходит, если объявленный маркер отсутствует, путь лишний
/// или маркер не объявлен (замена прежнего общего счётчика).
#[test]
fn l05_self_check_rejects_missing_and_undeclared_markers() {
    let p = compiled(l05_ir(vec![rule("MK-POL-RULE-DET", 10, Action::Alert, count("MK-POL-DETECTOR", 1))]));
    let i = sensitive_like_complete();
    let out = encode_decision(&crate::Pdp::decide(&p, &ctx(), &i, &valid(&[])).expect("решение"));
    let base: &[(&str, &[&str])] = &[
        (
            "MK-POL-RULE-DET",
            &[
                "winner_rule_id",
                "matched_rules[]",
                "action_sources[].source.rule_id",
                "explanation.rules[].rule_id",
                "explanation.possible_winners[]",
            ],
        ),
        ("MK-POL-DETECTOR", &["explanation.detectors[].detector_id", "explanation.rules[].determining_atom"]),
        ("MK-POL-BUNDLE", &["policy_bundle_id", "explanation.bundle_id"]),
        ("MK-POL-IIA", &[]),
    ];
    l05_check("L-05 базовый", &out, base);
    // IIA объявлена как присутствующая, но не активирована — должен быть FAIL.
    let missing: &[(&str, &[&str])] =
        &[base[0], base[1], base[2], ("MK-POL-IIA", &["incomplete_action_source"])];
    assert!(std::panic::catch_unwind(|| l05_check("missing", &out, missing)).is_err());
    // Маркер детектора не объявлен — должен быть FAIL.
    let undeclared: &[(&str, &[&str])] = &[base[0], base[2], base[3]];
    assert!(std::panic::catch_unwind(|| l05_check("undeclared", &out, undeclared)).is_err());
    // Неполный набор путей — должен быть FAIL.
    let partial: &[(&str, &[&str])] = &[("MK-POL-RULE-DET", &["winner_rule_id"]), base[1], base[2], base[3]];
    assert!(std::panic::catch_unwind(|| l05_check("partial", &out, partial)).is_err());
}

fn sensitive_like_complete() -> InspectionResult {
    insp(
        InspectionStatus::Match,
        &[],
        TextCoverage::Complete,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[],
        vec![det("MK-POL-DETECTOR", 2, true)],
        "TXT",
    )
}
