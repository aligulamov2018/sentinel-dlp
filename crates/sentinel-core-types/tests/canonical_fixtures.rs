//! CHG-0008: каноническое представление на уровне типов (матрица проверок, часть C).
//!
//! Ожидаемые байты — рукописные эталоны пакета CHG-0008 v0.2.1 (`tests/fixtures/canonical/`),
//! скопированные побайтно; они не порождаются кодировщиком. Входы — приложение A матрицы.
//! Значения строятся заново в каждом тесте; часть из них нарушает инварианты PDP намеренно:
//! кодировщик полный (CF-15).

#![cfg(feature = "canonical")]

use sentinel_core_types::canonical::{encode_decision, encode_inspection_result};
use sentinel_core_types::*;

macro_rules! fixture {
    ($name:literal) => {
        include_bytes!(concat!("fixtures/canonical/", $name, ".json")).as_slice()
    };
}

fn rid(s: &str) -> RuleId {
    RuleId::new(s)
}
fn rec(s: &str) -> RecordId {
    RecordId::new(s)
}

/// Сравнение с эталоном; при расхождении печатает обе строки (эталон не правится).
fn assert_bytes(name: &str, got: &[u8], expected: &[u8]) {
    assert!(
        got == expected,
        "{name}: вывод не совпадает с утверждённым эталоном\n  ожидалось: {}\n  получено:  {}",
        String::from_utf8_lossy(expected),
        String::from_utf8_lossy(got)
    );
}

// ---------------------------------------------------------------- входы F-D1…F-D3 (Decision)

pub fn f_d1() -> Decision {
    Decision {
        verdict: Verdict::Allow,
        reasons: vec![
            ReasonCode::ParseError,
            ReasonCode::Timeout,
            ReasonCode::ContentChanged,
            ReasonCode::PolicyAbsent,
            ReasonCode::UnsupportedFormat,
            ReasonCode::EngineError,
            ReasonCode::PolicyInvalid,
            ReasonCode::ContentEncrypted,
            ReasonCode::ResourceExhausted,
            ReasonCode::PolicyExpired,
            ReasonCode::Timeout,
        ],
        policy_version: Some(u64::MAX),
        policy_bundle_id: Some(BundleId::new("bundle-\u{fc}")),
        matched_rules: vec![rid("r-b"), rid("r-a")],
        required_action: Some(Action::Alert),
        decision_basis: DecisionBasis::OutcomeIndependentOfUnknown,
        inspection_complete: false,
        winner_rule_id: Some(rid("r-b")),
        possible_winners: vec![rid("r-z"), rid("r-a")],
        part_exclusion_id: Some(rec("ex-1")),
        precedence_approval_ids: vec![rec("ap-2"), rec("ap-1")],
        suppressed_precedence_pairs: vec![(rid("r-hi"), rid("r-lo"))],
        inactive_deviation_ids: vec![rec("dv-1")],
        deviation_input: DeviationInput::Contradictory,
        incomplete_action_source: Some(rec("iia-1")),
        action_certainty: Some(ActionCertainty::Possible),
        action_sources: vec![
            ActionSource { source: ActionSourceRef::Rule(rid("r-b")), action: Action::Alert },
            ActionSource {
                source: ActionSourceRef::IncompleteInspectionAction(rec("iia-1")),
                action: Action::Block,
            },
            ActionSource { source: ActionSourceRef::Rule(rid("r-c")), action: Action::Monitor },
        ],
        verification_mode: Some(VerificationMode::InspectionOnly),
        capability_check: Some(CapabilityCheck::Skipped),
        requested_action: Some(Action::Block),
        capability_exception_id: Some(rec("cx-1")),
        fallback: Some(Fallback::AllowWithAudit),
        data_level: DataLevel::MaskedMatch,
        degraded: true,
        explanation: Explanation {
            core_version: "0.1.0".into(),
            detector_set_version: "ds-1".into(),
            bundle_id: Some(BundleId::new("bundle-\u{fc}")),
            policy_version: Some(9_007_199_254_740_993),
            file_class: Some(FileClass::new("OOXML")),
            classification: Some(Classification::Ambiguous(AmbiguityReason::EmbeddedSignature)),
            text_coverage: Some(TextCoverage::Partial),
            structure_enumeration: Some(StructureEnumeration::Incomplete(ReasonCode::ResourceExhausted)),
            inspection_complete: false,
            detectors: vec![(DetectorId::new("det-b"), u32::MAX, true), (DetectorId::new("det-a"), 0, false)],
            rules: vec![
                RuleTrace {
                    rule_id: rid("r-b"),
                    value: Truth::True,
                    determining_atom: Some("detector(det-b).count >= 1".into()),
                    guards_incompleteness: false,
                },
                RuleTrace {
                    rule_id: rid("r-a"),
                    value: Truth::Unknown,
                    determining_atom: None,
                    guards_incompleteness: true,
                },
                RuleTrace {
                    rule_id: rid("r-c"),
                    value: Truth::False,
                    determining_atom: Some("inspection.incomplete(TIMEOUT)".into()),
                    guards_incompleteness: false,
                },
            ],
            winner_basis: Some(OrderBasis::RuleId),
            unknown_rules: vec![rid("r-a")],
            possible_winners: vec![rid("r-b"), rid("r-a")],
            world_without_winner: true,
            worlds_considered: u64::MAX,
            class_worlds: vec![Some(FileClass::new("TXT")), None, Some(FileClass::new("OOXML"))],
            fallback_sources: vec![rid("r-b")],
            part_exclusion_id: Some(rec("ex-1")),
            precedence_approval_ids: vec![rec("ap-1")],
            suppressed_precedence_pairs: vec![(rid("r-hi"), rid("r-lo"))],
            incomplete_action_source: Some(rec("iia-1")),
            capability_exception_id: Some(rec("cx-1")),
            shadowed_incompleteness_guards: vec![rid("r-a")],
            deviation_contradictions: vec!["TIME_UNVERIFIABLE при непустом valid_ids".into()],
            policy_state: Some(ReasonCode::PolicyExpired),
        },
        schema_version: u32::MAX,
    }
}

pub fn f_d2() -> Decision {
    Decision {
        verdict: Verdict::Block,
        reasons: vec![],
        policy_version: None,
        policy_bundle_id: None,
        matched_rules: vec![],
        required_action: None,
        decision_basis: DecisionBasis::NoRuleMatched,
        inspection_complete: false,
        winner_rule_id: None,
        possible_winners: vec![],
        part_exclusion_id: None,
        precedence_approval_ids: vec![],
        suppressed_precedence_pairs: vec![],
        inactive_deviation_ids: vec![],
        deviation_input: DeviationInput::Provided,
        incomplete_action_source: None,
        action_certainty: None,
        action_sources: vec![],
        verification_mode: None,
        capability_check: None,
        requested_action: None,
        capability_exception_id: None,
        fallback: None,
        data_level: DataLevel::Metadata,
        degraded: false,
        explanation: Explanation {
            core_version: String::new(),
            detector_set_version: String::new(),
            ..Explanation::default()
        },
        schema_version: 0,
    }
}

pub fn f_d3() -> Decision {
    let mut d = f_d2();
    d.matched_rules = [
        "q\"",
        "b\\",
        "s/",
        "n\u{0}",
        "c\u{1}\u{1f}",
        "w\u{8}\t\n\u{c}\r",
        "d\u{7f}",
        "c1\u{80}",
        "\u{e9}",
        "e\u{301}",
        "ls\u{2028}",
        "emoji\u{1f600}",
        "bom\u{feff}",
        "",
    ]
    .iter()
    .map(|s| rid(s))
    .collect();
    d.explanation.deviation_contradictions = vec!["строка\nвторая".into()];
    d
}

// ---------------------------------------------------------------- входы F-I1, F-I2 (InspectionResult)

pub fn f_i1() -> InspectionResult {
    let mut full = [0u8; 32];
    for (i, b) in full.iter_mut().enumerate() {
        *b = u8::try_from(i).unwrap_or(0);
    }
    InspectionResult {
        status: InspectionStatus::Match,
        reasons: vec![ReasonCode::ResourceExhausted, ReasonCode::Timeout],
        text_coverage: TextCoverage::Partial,
        structure_enumeration: StructureEnumeration::Incomplete(ReasonCode::ParseError),
        classification: Classification::Ambiguous(AmbiguityReason::DeclaredContainerMismatch),
        uninspected_parts: vec![
            UninspectedPart { kind: PartKind::Unknown, count: u32::MAX },
            UninspectedPart { kind: PartKind::Image, count: 1 },
            UninspectedPart { kind: PartKind::Attachment, count: 0 },
            UninspectedPart { kind: PartKind::EmbeddedObject, count: 2 },
        ],
        detections: vec![
            DetectionResult {
                detector_id: DetectorId::new("det-a"),
                detector_version: "1.2.3\t\u{1f}".into(),
                match_count: 0,
                score: u32::MAX,
                threshold_met: false,
            },
            DetectionResult {
                detector_id: DetectorId::new("det-b"),
                detector_version: String::new(),
                match_count: u32::MAX,
                score: 0,
                threshold_met: true,
            },
        ],
        file_class: FileClass::new("OOXML"),
        digest: ContentDigest {
            full: Some(full),
            full_status: DigestStatus::Computed,
            read_set_digest: [0xff; 32],
            trust: DigestTrust::TrustedSnapshot,
        },
        engine: EngineInfo {
            core_version: "0.1.0".into(),
            detector_set_version: "ds-1".into(),
            limits_profile_id: "lim-\u{fc}\u{2028}".into(),
        },
        inspection_origin: InspectionOrigin::HostSynthesized,
    }
}

pub fn f_i2() -> InspectionResult {
    InspectionResult {
        status: InspectionStatus::NoMatch,
        reasons: vec![],
        text_coverage: TextCoverage::Complete,
        structure_enumeration: StructureEnumeration::Complete,
        classification: Classification::Confirmed,
        uninspected_parts: vec![],
        detections: vec![],
        file_class: FileClass::new(""),
        digest: ContentDigest::not_computed(DigestStatus::NoSnapshot),
        engine: EngineInfo {
            core_version: String::new(),
            detector_set_version: String::new(),
            limits_profile_id: String::new(),
        },
        inspection_origin: InspectionOrigin::Core,
    }
}

// ---------------------------------------------------------------- C-01…C-05

#[test]
fn c01_decision_max() {
    assert_bytes("F-D1", &encode_decision(&f_d1()), fixture!("F-D1-decision-max"));
}

#[test]
fn c02_decision_min() {
    assert_bytes("F-D2", &encode_decision(&f_d2()), fixture!("F-D2-decision-min"));
}

#[test]
fn c03_decision_strings() {
    assert_bytes("F-D3", &encode_decision(&f_d3()), fixture!("F-D3-decision-strings"));
}

#[test]
fn c04_inspection_max() {
    assert_bytes("F-I1", &encode_inspection_result(&f_i1()), fixture!("F-I1-inspection-max"));
}

#[test]
fn c05_inspection_min() {
    assert_bytes("F-I2", &encode_inspection_result(&f_i2()), fixture!("F-I2-inspection-min"));
}

// ---------------------------------------------------------------- C-06…C-20 (V-01…V-15)

fn d_variant(base: fn() -> Decision, f: impl FnOnce(&mut Decision)) -> Vec<u8> {
    let mut d = base();
    f(&mut d);
    encode_decision(&d)
}

fn i_variant(f: impl FnOnce(&mut InspectionResult)) -> Vec<u8> {
    let mut r = f_i2();
    f(&mut r);
    encode_inspection_result(&r)
}

/// Варианты V-01…V-15: идентификатор, свежий вывод кодировщика, утверждённый эталон.
fn v_cases() -> Vec<(&'static str, Vec<u8>, &'static [u8])> {
    vec![
        (
            "V-01",
            d_variant(f_d2, |d| d.verdict = Verdict::Indeterminate),
            fixture!("V-01-verdict-indeterminate"),
        ),
        (
            "V-02",
            d_variant(f_d2, |d| d.decision_basis = DecisionBasis::RuleMatched),
            fixture!("V-02-basis-rule-matched"),
        ),
        (
            "V-03",
            d_variant(f_d2, |d| d.decision_basis = DecisionBasis::Indeterminate),
            fixture!("V-03-basis-indeterminate"),
        ),
        ("V-04", d_variant(f_d1, |d| d.fallback = Some(Fallback::Block)), fixture!("V-04-fallback-block")),
        (
            "V-05",
            d_variant(f_d1, |d| d.action_certainty = Some(ActionCertainty::Definite)),
            fixture!("V-05-certainty-definite"),
        ),
        (
            "V-06",
            d_variant(f_d1, |d| d.verification_mode = Some(VerificationMode::EnforcementReady)),
            fixture!("V-06-mode-enforcement-ready"),
        ),
        (
            "V-07",
            d_variant(f_d1, |d| d.capability_check = Some(CapabilityCheck::Performed)),
            fixture!("V-07-capability-performed"),
        ),
        (
            "V-08",
            d_variant(f_d2, |d| d.deviation_input = DeviationInput::Unavailable),
            fixture!("V-08-deviation-unavailable"),
        ),
        (
            "V-09",
            d_variant(f_d1, |d| d.explanation.winner_basis = Some(OrderBasis::Priority)),
            fixture!("V-09-basis-priority"),
        ),
        (
            "V-10",
            d_variant(f_d1, |d| d.explanation.winner_basis = Some(OrderBasis::Strictness)),
            fixture!("V-10-basis-strictness"),
        ),
        (
            "V-11",
            i_variant(|r| r.status = InspectionStatus::NotInspected),
            fixture!("V-11-status-not-inspected"),
        ),
        (
            "V-12",
            i_variant(|r| r.digest.full_status = DigestStatus::PartialRead),
            fixture!("V-12-digest-partial-read"),
        ),
        (
            "V-13",
            i_variant(|r| r.digest.full_status = DigestStatus::SizeLimit),
            fixture!("V-13-digest-size-limit"),
        ),
        (
            "V-14",
            i_variant(|r| r.digest.full_status = DigestStatus::ReadError),
            fixture!("V-14-digest-read-error"),
        ),
        (
            "V-15",
            i_variant(|r| r.digest.full_status = DigestStatus::NotRequested),
            fixture!("V-15-digest-not-requested"),
        ),
    ]
}

#[test]
fn c06_c20_variants() {
    let cases = v_cases();
    assert_eq!(cases.len(), 15);
    for (name, got, expected) in &cases {
        assert_bytes(name, got, expected);
    }
}

// ---------------------------------------------------------------- C-23 и свойства кодировщика

#[test]
fn c23_repeated_encoding_is_identical() {
    let d = f_d1();
    let i = f_i1();
    let d0 = encode_decision(&d);
    let i0 = encode_inspection_result(&i);
    for _ in 0..1000 {
        assert_eq!(encode_decision(&d), d0);
        assert_eq!(encode_inspection_result(&i), i0);
    }
    // Кодирование не меняет значение.
    assert_eq!(d, f_d1());
    assert_eq!(i, f_i1());
}

#[test]
fn encoder_does_not_sort_or_dedup() {
    // CF-5: порядок значения и повторы сохраняются (раздел 6 контракта).
    let mut d = f_d2();
    d.reasons = vec![ReasonCode::Timeout, ReasonCode::ContentChanged, ReasonCode::Timeout];
    d.possible_winners = vec![rid("b"), rid("a"), rid("b")];
    let s = String::from_utf8(encode_decision(&d)).unwrap_or_default();
    assert!(s.contains(r#""reasons":["TIMEOUT","CONTENT_CHANGED","TIMEOUT"]"#), "{s}");
    assert!(s.contains(r#""possible_winners":["b","a","b"]"#), "{s}");
}

#[test]
fn every_control_character_is_escaped() {
    // CF-12: все U+0000…U+001F — экранированы; U+007F — нет.
    let all: String = (0u8..0x20).map(char::from).chain(std::iter::once('\u{7f}')).collect();
    let mut r = f_i2();
    r.engine.limits_profile_id = all;
    let out = encode_inspection_result(&r);
    assert!(out.iter().all(|&b| b >= 0x20), "сырой управляющий байт в выводе");
    let s = String::from_utf8(out).unwrap_or_default();
    let expected = concat!(
        r#""limits_profile_id":""#,
        r"\u0000\u0001\u0002\u0003\u0004\u0005\u0006\u0007\b\t\n\u000b\f\r\u000e\u000f",
        r"\u0010\u0011\u0012\u0013\u0014\u0015\u0016\u0017\u0018\u0019\u001a\u001b\u001c\u001d\u001e\u001f",
        "\u{7f}\""
    );
    assert!(s.contains(expected), "{s}");
}

/// L-01 (часть `InspectionResult`): в каждое строковое поле — свой маркер. Каждый маркер
/// встречается ровно один раз и только как значение своего ключа; ключи внутри
/// `InspectionResult` уникальны, поэтому пара «ключ: значение» однозначно задаёт путь.
/// Строковые поля `Decision` проверяет L-01 в `sentinel-core-pdp` (тестовый разбор JSON).
#[test]
fn l01_inspection_result_string_slots() {
    let mut r = f_i1();
    r.file_class = FileClass::new("MK-L01-file_class");
    r.detections[0].detector_id = DetectorId::new("MK-L01-detector_id-0");
    r.detections[0].detector_version = "MK-L01-detector_version-0".into();
    r.detections[1].detector_id = DetectorId::new("MK-L01-detector_id-1");
    r.detections[1].detector_version = "MK-L01-detector_version-1".into();
    r.engine.core_version = "MK-L01-core_version".into();
    r.engine.detector_set_version = "MK-L01-detector_set_version".into();
    r.engine.limits_profile_id = "MK-L01-limits_profile_id".into();
    let out = String::from_utf8(encode_inspection_result(&r)).unwrap_or_default();
    let slots = [
        ("file_class", "MK-L01-file_class"),
        ("detector_id", "MK-L01-detector_id-0"),
        ("detector_version", "MK-L01-detector_version-0"),
        ("detector_id", "MK-L01-detector_id-1"),
        ("detector_version", "MK-L01-detector_version-1"),
        ("core_version", "MK-L01-core_version"),
        ("detector_set_version", "MK-L01-detector_set_version"),
        ("limits_profile_id", "MK-L01-limits_profile_id"),
    ];
    for (key, marker) in slots {
        assert_eq!(out.matches(&format!("\"{marker}\"")).count(), 1, "{marker}: не ровно одно вхождение");
        assert_eq!(
            out.matches(&format!("\"{key}\":\"{marker}\"")).count(),
            1,
            "{marker}: не по своему ключу"
        );
    }
    assert_eq!(out.matches("MK-").count(), slots.len(), "лишние или перенесённые маркеры");
}

// ---------------------------------------------------------------- F-02, F-03 (C-фикстуры)

type DecisionFactory = fn() -> Decision;
type InspectionFactory = fn() -> InspectionResult;

const C_DECISIONS: &[(&str, DecisionFactory)] = &[("F-D1", f_d1), ("F-D2", f_d2), ("F-D3", f_d3)];
const C_INSPECTIONS: &[(&str, InspectionFactory)] = &[("F-I1", f_i1), ("F-I2", f_i2)];

/// Вывод всех 20 C-фикстур (C-01…C-20: F-D1…F-I2, V-01…V-15); каждое значение строится заново.
fn c_outputs() -> Vec<(&'static str, Vec<u8>)> {
    let mut v: Vec<(&'static str, Vec<u8>)> =
        C_DECISIONS.iter().map(|(n, f)| (*n, encode_decision(&f()))).collect();
    v.extend(C_INSPECTIONS.iter().map(|(n, f)| (*n, encode_inspection_result(&f()))));
    v.extend(v_cases().into_iter().map(|(n, got, _)| (n, got)));
    v
}

/// Запись вывода `<каталог>/<ID>.json`; суммы вычисляет внешняя утилита.
fn write_outputs(dir: &std::path::Path, outputs: &[(&str, Vec<u8>)]) {
    std::fs::create_dir_all(dir).expect("каталог вывода");
    for (name, bytes) in outputs {
        std::fs::write(dir.join(format!("{name}.json")), bytes).expect("запись вывода");
    }
}

/// F-01 (C): 1000 повторов в одном процессе; каждый повтор строит все значения заново.
#[test]
fn f01_c_cases_repeat_in_process() {
    let first = c_outputs();
    assert_eq!(first.len(), 20);
    for _ in 0..1000 {
        assert_eq!(c_outputs(), first, "повтор дал другие байты");
    }
}

/// F-02 (C): 1 поток против 8 потоков; каждый поток строит значения сам. При заданной
/// `SENTINEL_CANONICAL_THREADS_OUT` вывод пишется в `single/` и `thread-<k>/` этого каталога.
#[test]
fn f02_c_cases_threads() {
    let single = c_outputs();
    let per_thread: Vec<_> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..8).map(|_| s.spawn(c_outputs)).collect();
        handles.into_iter().map(|h| h.join().expect("поток проверки завершился паникой")).collect()
    });
    if let Some(dir) = std::env::var_os("SENTINEL_CANONICAL_THREADS_OUT") {
        let dir = std::path::PathBuf::from(dir);
        write_outputs(&dir.join("single"), &single);
        for (k, outputs) in per_thread.iter().enumerate() {
            write_outputs(&dir.join(format!("thread-{k}")), outputs);
        }
    }
    for (k, outputs) in per_thread.iter().enumerate() {
        assert_eq!(outputs, &single, "поток {k}: байты отличаются от однопоточного прогона");
    }
}

/// F-03, F-04, F-05: запись вывода C-фикстур в каталог `SENTINEL_CANONICAL_OUT` (если задан).
/// Контрольные суммы вычисляет внешняя утилита по записанным файлам.
#[test]
fn f03_emit_c_outputs() {
    let Some(dir) = std::env::var_os("SENTINEL_CANONICAL_OUT") else { return };
    write_outputs(std::path::Path::new(&dir), &c_outputs());
}
