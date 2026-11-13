//! Golden-ожидания G-01…G-30 (docs/architecture/SENTINEL_CORE_TEST_PLAN.md §5).
//! Ожидания записаны явно по таблице; функции алгоритма для их вычисления не используются.
//! Во всех случаях контекст — канал USB, `on_indeterminate = ALLOW_WITH_AUDIT`, контракт
//! неполной инспекции — MONITOR без ограничения области, если не указано иное.
//!
//! G-26…G-30 описывают протокол хоста (момент вычисления `DeviationValidity`, дедлайн,
//! предварительное решение). PDP получает только итоговый вход; вычисление его хостом на
//! этапе 1 не реализовано и здесь не проверяется (NOT VERIFIED).

use super::*;
use crate::policy::{ConflictClass, IncompleteAction};
use crate::BundleError;
use sentinel_core_types::ActionSourceRef as Src;

fn src_rule(id: &str, a: Action) -> ActionSource {
    ActionSource { source: Src::Rule(rid(id)), action: a }
}
fn src_iia(id: &str, a: Action) -> ActionSource {
    ActionSource { source: Src::IncompleteInspectionAction(rec(id)), action: a }
}

#[test]
fn g01_unknown_rule_gives_indeterminate() {
    let p = compiled(ir(vec![rule("R1", 10, Action::Block, card(1))]));
    let d = decide(&p, &not_inspected(ReasonCode::ResourceExhausted), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.reasons, vec![ReasonCode::ResourceExhausted]);
    assert_eq!(d.fallback, Some(Fallback::AllowWithAudit));
    assert_eq!(d.required_action, Some(Action::Monitor));
    assert_eq!(d.explanation.unknown_rules, vec![rid("R1")]);
}

#[test]
fn g02_context_only_rule_is_definite() {
    let p = compiled(ir(vec![rule("R2", 10, Action::Alert, usb())]));
    let d = decide(&p, &not_inspected(ReasonCode::UnsupportedFormat), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Allow);
    assert_eq!(d.required_action, Some(Action::Alert));
    assert_eq!(d.action_certainty, Some(ActionCertainty::Definite));
    assert_eq!(d.decision_basis, DecisionBasis::RuleMatched);
    assert_eq!(d.winner_rule_id, Some(rid("R2")));
}

#[test]
fn g03_incompleteness_guard_blocks() {
    let p = compiled(ir(vec![rule("R3", 10, Action::Block, incomplete(ReasonCode::ContentEncrypted))]));
    let d = decide(&p, &not_inspected(ReasonCode::ContentEncrypted), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Block);
    assert_eq!(d.winner_rule_id, Some(rid("R3")));
}

#[test]
fn g04_partial_text_match_blocks() {
    let i = partial_text(ReasonCode::ResourceExhausted, 5);
    assert_eq!(i.status, InspectionStatus::Match);
    assert_eq!(i.text_coverage, TextCoverage::Partial);
    let p = compiled(ir(vec![rule("R1", 10, Action::Block, card(1))]));
    let d = decide(&p, &i, &valid(&[]));
    assert_eq!(d.verdict, Verdict::Block);
    assert_eq!(d.winner_rule_id, Some(rid("R1")));
}

#[test]
fn g05_threshold_not_reached_is_indeterminate() {
    let p = compiled(ir(vec![rule("R4", 10, Action::Block, card(10))]));
    let d = decide(&p, &partial_text(ReasonCode::ResourceExhausted, 2), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.reasons, vec![ReasonCode::ResourceExhausted]);
    assert_eq!(d.fallback, Some(Fallback::AllowWithAudit));
    assert_eq!(d.explanation.unknown_rules, vec![rid("R4")]);
}

fn g06_policy(with_e1: bool) -> CompiledPolicy<InspectionOnly> {
    let mut ir = ir(vec![rule("Rc", 10, Action::Block, card(1))]);
    if with_e1 {
        ir.exclusions.push(e1(None));
    }
    compiled(ir)
}

#[test]
fn g06_exclusion_applied() {
    let i = docx_with_image(0);
    assert_eq!(i.status, InspectionStatus::NotInspected);
    assert_eq!(i.reasons, vec![ReasonCode::UnsupportedFormat]);
    let d = decide(&g06_policy(true), &i, &valid(&["E1"]));
    assert_eq!(d.verdict, Verdict::Allow);
    assert_eq!(d.decision_basis, DecisionBasis::OutcomeIndependentOfUnknown);
    assert_eq!(d.required_action, Some(Action::Monitor));
    assert_eq!(d.part_exclusion_id, Some(rec("E1")));
    assert_eq!(d.incomplete_action_source, Some(rec("IIA-default")));
    // Без E1 → INDETERMINATE(UNSUPPORTED_FORMAT).
    let d = decide(&g06_policy(false), &i, &valid(&[]));
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.reasons, vec![ReasonCode::UnsupportedFormat]);
}

#[test]
fn g07_damaged_structure_disables_exclusion() {
    let i = insp(
        InspectionStatus::NotInspected,
        &[ReasonCode::ParseError],
        TextCoverage::Complete,
        StructureEnumeration::Incomplete(ReasonCode::ParseError),
        Classification::Confirmed,
        &[PartKind::Image],
        vec![det("card", 0, false)],
        "OOXML",
    );
    let d = decide(&g06_policy(true), &i, &valid(&["E1"]));
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.fallback, Some(Fallback::AllowWithAudit));
    assert_eq!(d.part_exclusion_id, None);
}

#[test]
fn g08_full_coverage_no_rule_matched() {
    let i = insp(
        InspectionStatus::NoMatch,
        &[],
        TextCoverage::Complete,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[],
        vec![det("card", 0, false)],
        "OOXML",
    );
    let d = decide(&g06_policy(false), &i, &valid(&[]));
    assert_eq!(d.verdict, Verdict::Allow);
    assert_eq!(d.decision_basis, DecisionBasis::NoRuleMatched);
    assert_eq!(d.required_action, None);
    // Действия нет — нет и контракта действия (толкование этапа 1, см. отчёт: INV-C13 и G-08).
    assert_eq!(d.action_certainty, None);
    assert!(d.action_sources.is_empty());
}

#[test]
fn g09_or_with_known_true_branch() {
    let p = compiled(ir(vec![rule("R8", 10, Action::Block, Expr::Or(vec![usb(), card(1)]))]));
    let d = decide(&p, &not_inspected(ReasonCode::Timeout), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Block);
    assert_eq!(d.winner_rule_id, Some(rid("R8")));
}

fn g10_rules() -> Vec<Rule> {
    vec![
        rule("R9", 10, Action::Alert, usb()),
        rule("R10", 5, Action::Block, incomplete(ReasonCode::ContentEncrypted)),
    ]
}

#[test]
fn g10_conflict_b_requires_approval() {
    let err = validate(ir(g10_rules())).unwrap_err();
    assert_eq!(
        err,
        BundleError::UnapprovedPrecedenceConflict {
            higher: rid("R9"),
            lower: rid("R10"),
            class: ConflictClass::B
        }
    );
    let mut with = ir(g10_rules());
    with.approvals.push(pca("A1", "R9", "R10"));
    let d = decide(&compiled(with), &not_inspected(ReasonCode::ContentEncrypted), &valid(&["A1"]));
    assert_eq!(d.verdict, Verdict::Allow);
    assert_eq!(d.required_action, Some(Action::Alert));
    assert_eq!(d.winner_rule_id, Some(rid("R9")));
    assert_eq!(d.precedence_approval_ids, vec![rec("A1")]);
}

#[test]
fn g11_incompleteness_atom_true_despite_match() {
    let i = partial_text(ReasonCode::ResourceExhausted, 1);
    assert_eq!(i.status, InspectionStatus::Match);
    let p = compiled(ir(vec![
        rule("R11", 20, Action::Block, incomplete(ReasonCode::ResourceExhausted)),
        rule("R12", 10, Action::Alert, card(1)),
    ]));
    let d = decide(&p, &i, &valid(&[]));
    assert_eq!(d.verdict, Verdict::Block);
    assert_eq!(d.winner_rule_id, Some(rid("R11")));
    // Обратные приоритеты — конфликт, бандл без утверждения отклоняется.
    let err = validate(ir(vec![
        rule("R12", 20, Action::Alert, card(1)),
        rule("R11", 10, Action::Block, incomplete(ReasonCode::ResourceExhausted)),
    ]))
    .unwrap_err();
    assert!(
        matches!(err, BundleError::UnapprovedPrecedenceConflict { ref higher, ref lower, .. } if *higher == rid("R12") && *lower == rid("R11"))
    );
}

#[test]
fn g12_exclusion_does_not_hide_found_match() {
    let i = docx_with_image(1);
    let d = decide(&g06_policy(true), &i, &valid(&["E1"]));
    assert_eq!(d.verdict, Verdict::Block);
    assert_eq!(d.winner_rule_id, Some(rid("Rc")));
    assert_eq!(d.part_exclusion_id, Some(rec("E1")));
}

fn g13_rules(r13_block_fallback: bool) -> Vec<Rule> {
    let r13 = rule("R13", 5, Action::Block, card(1));
    vec![rule("R14", 10, Action::Alert, card(10)), if r13_block_fallback { on_ind_block(r13) } else { r13 }]
}

#[test]
fn g13_conflict_a_and_two_outcomes() {
    let err = validate(ir(g13_rules(false))).unwrap_err();
    assert_eq!(
        err,
        BundleError::UnapprovedPrecedenceConflict {
            higher: rid("R14"),
            lower: rid("R13"),
            class: ConflictClass::A
        }
    );
    let mut with = ir(g13_rules(false));
    with.approvals.push(pca("A1", "R14", "R13"));
    let d = decide(&compiled(with), &partial_text(ReasonCode::Timeout, 1), &valid(&["A1"]));
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.reasons, vec![ReasonCode::Timeout]);
    assert_eq!(d.fallback, Some(Fallback::AllowWithAudit));
    assert_eq!(d.precedence_approval_ids, vec![rec("A1")]);
}

#[test]
fn g14_block_fallback_when_block_is_possible() {
    let mut with = ir(g13_rules(true));
    with.approvals.push(pca("A1", "R14", "R13"));
    let d = decide(&compiled(with), &partial_text(ReasonCode::Timeout, 1), &valid(&["A1"]));
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.fallback, Some(Fallback::Block));
    assert_eq!(d.required_action, Some(Action::Block));
}

#[test]
fn g15_two_allow_outcomes_no_false_block() {
    let p = compiled(ir(vec![
        on_ind_block(rule("R16", 10, Action::Monitor, card(1))),
        rule("R15", 5, Action::Alert, usb()),
    ]));
    let d = decide(&p, &not_inspected(ReasonCode::Timeout), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Allow);
    assert_eq!(d.required_action, Some(Action::Alert));
    assert_eq!(d.action_certainty, Some(ActionCertainty::Possible));
    assert_eq!(d.action_sources, vec![src_rule("R16", Action::Monitor), src_rule("R15", Action::Alert)]);
    assert_eq!(d.winner_rule_id, None);
    assert_eq!(d.possible_winners, vec![rid("R15"), rid("R16")]);
    assert_eq!(d.fallback, None);
}

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

#[test]
fn g16_ambiguous_classification_disables_exclusion() {
    let i = ambiguous_text("TXT");
    assert_eq!(i.status, InspectionStatus::NotInspected);
    let d = decide(&g06_policy(true), &i, &valid(&["E1"]));
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.reasons, vec![ReasonCode::UnsupportedFormat]);
    assert_eq!(d.fallback, Some(Fallback::AllowWithAudit));
    assert_eq!(d.part_exclusion_id, None);
}

#[test]
fn g17_unsatisfiable_rule_gives_no_false_block() {
    let p = compiled(ir(vec![
        on_ind_block(rule("R25", 30, Action::Block, Expr::And(vec![card(5), not(card(3))]))),
        rule("R26", 10, Action::Alert, usb()),
    ]));
    let d = decide(&p, &not_inspected(ReasonCode::Timeout), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Allow);
    assert_eq!(d.required_action, Some(Action::Alert));
    assert_eq!(d.winner_rule_id, Some(rid("R26")));
}

#[test]
fn g18_conflict_a_rejected() {
    let err = validate(ir(vec![
        rule("Rb", 30, Action::Block, not(card(5))),
        rule("Rh", 20, Action::Alert, card(5)),
        on_ind_block(rule("Rl", 10, Action::Block, usb())),
    ]))
    .unwrap_err();
    assert_eq!(
        err,
        BundleError::UnapprovedPrecedenceConflict {
            higher: rid("Rh"),
            lower: rid("Rl"),
            class: ConflictClass::A
        }
    );
}

#[test]
fn g19_contract_action_alert_from_scoped_record() {
    let mut p = ir(vec![rule("R30", 10, Action::Monitor, card(1))]);
    p.incomplete_actions.push(iia(
        "IIA-usb",
        Scope { channels: vec![Channel::new("USB")], ..Scope::default() },
        IncompleteAction::Alert,
    ));
    let d = decide(&compiled(p), &not_inspected(ReasonCode::Timeout), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Allow);
    assert_eq!(d.decision_basis, DecisionBasis::OutcomeIndependentOfUnknown);
    assert_eq!(d.required_action, Some(Action::Alert));
    assert_eq!(d.action_certainty, Some(ActionCertainty::Possible));
    assert_eq!(d.action_sources, vec![src_rule("R30", Action::Monitor), src_iia("IIA-usb", Action::Alert)]);
    assert_eq!(d.incomplete_action_source, Some(rec("IIA-usb")));
}

#[test]
fn g20_file_class_worlds_when_ambiguous() {
    let rules = || {
        vec![
            rule("R40", 10, Action::Alert, class_is("TXT")),
            rule("R41", 5, Action::Block, class_is("OOXML")),
        ]
    };
    // Доказуемое непересечение по file.class — бандл загружается без утверждения.
    let p = compiled(ir(rules()));
    let d = decide(&p, &ambiguous_text("TXT"), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.reasons, vec![ReasonCode::UnsupportedFormat]);
    assert_eq!(d.fallback, Some(Fallback::AllowWithAudit));
    assert_eq!(
        d.explanation.class_worlds,
        vec![Some(FileClass::new("OOXML")), Some(FileClass::new("TXT")), None]
    );
    // Тот же текст с расширением .txt (CONFIRMED) → ALLOW, ALERT (R40).
    let confirmed = insp(
        InspectionStatus::NoMatch,
        &[],
        TextCoverage::Complete,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[],
        vec![det("card", 0, false)],
        "TXT",
    );
    let d = decide(&p, &confirmed, &valid(&[]));
    assert_eq!(d.verdict, Verdict::Allow);
    assert_eq!(d.required_action, Some(Action::Alert));
    assert_eq!(d.winner_rule_id, Some(rid("R40")));
    // Вариант R42/R43 — конфликт (a), без утверждения отклоняется.
    let err = validate(ir(vec![
        rule("R42", 10, Action::Alert, class_is("OOXML")),
        rule("R43", 5, Action::Block, usb()),
    ]))
    .unwrap_err();
    assert_eq!(
        err,
        BundleError::UnapprovedPrecedenceConflict {
            higher: rid("R42"),
            lower: rid("R43"),
            class: ConflictClass::A
        }
    );
}

#[test]
fn g21_known_limitation_false_block_fallback() {
    let p = compiled(ir(vec![
        on_ind_block(rule("R50", 20, Action::Block, Expr::And(vec![count("kw", 1), not(card(1))]))),
        rule("R51", 10, Action::Alert, usb()),
    ]));
    let d = decide(&p, &not_inspected(ReasonCode::Timeout), &valid(&[]));
    // Модель независимых счётчиков: известный ложный fallback BLOCK (O-15).
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.reasons, vec![ReasonCode::Timeout]);
    assert_eq!(d.fallback, Some(Fallback::Block));
}

#[test]
fn g22_contract_action_does_not_replace_strictest() {
    let p = compiled(ir(vec![rule("R1", 10, Action::Alert, card(1))]));
    let d = decide(&p, &not_inspected(ReasonCode::Timeout), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Allow);
    assert_eq!(d.decision_basis, DecisionBasis::OutcomeIndependentOfUnknown);
    assert_eq!(d.required_action, Some(Action::Alert));
    assert_eq!(d.action_certainty, Some(ActionCertainty::Possible));
    assert_eq!(
        d.action_sources,
        vec![src_rule("R1", Action::Alert), src_iia("IIA-default", Action::Monitor)]
    );
}

fn g23_rules() -> Vec<Rule> {
    vec![rule("R1", 100, Action::Alert, usb()), rule("R2", 10, Action::Block, count("sensitive", 1))]
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

fn g23_policy() -> CompiledPolicy<InspectionOnly> {
    let mut p = ir(g23_rules());
    p.approvals.push(pca("A1", "R1", "R2"));
    compiled(p)
}

#[test]
fn g23_context_only_shadowing_is_class_c() {
    let err = validate(ir(g23_rules())).unwrap_err();
    assert_eq!(
        err,
        BundleError::UnapprovedPrecedenceConflict {
            higher: rid("R1"),
            lower: rid("R2"),
            class: ConflictClass::C
        }
    );
    let p = g23_policy();
    // Действительное утверждение, область покрывает контекст.
    let d = decide(&p, &sensitive_full(), &valid(&["A1"]));
    assert_eq!(d.verdict, Verdict::Allow);
    assert_eq!(d.required_action, Some(Action::Alert));
    assert_eq!(d.winner_rule_id, Some(rid("R1")));
    // Утверждение недействительно.
    let d = decide(&p, &sensitive_full(), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Block);
    assert_eq!(d.winner_rule_id, Some(rid("R2")));
    assert_eq!(d.suppressed_precedence_pairs, vec![(rid("R1"), rid("R2"))]);
    // Контекст вне области утверждения.
    let d = crate::Pdp::decide(&p, &ctx_group("other"), &sensitive_full(), &valid(&["A1"])).unwrap();
    assert_eq!(d.verdict, Verdict::Block);
    assert_eq!(d.winner_rule_id, Some(rid("R2")));
    assert_eq!(d.suppressed_precedence_pairs, vec![(rid("R1"), rid("R2"))]);
}

#[test]
fn g24_invalid_exclusion_equals_absent_record() {
    let i = docx_with_image(0);
    let d = decide(&g06_policy(true), &i, &valid(&[]));
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.reasons, vec![ReasonCode::UnsupportedFormat]);
    assert_eq!(d.fallback, Some(Fallback::AllowWithAudit));
    assert_eq!(d.inactive_deviation_ids, vec![rec("E1")]);
    let without = decide(&g06_policy(false), &i, &valid(&[]));
    assert_eq!(
        (d.verdict, d.fallback, d.required_action),
        (without.verdict, without.fallback, without.required_action)
    );
}

#[test]
fn g25_approval_expired_before_decide_suppresses_pair() {
    let d = decide(&g23_policy(), &sensitive_full(), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Block);
    assert_eq!(d.winner_rule_id, Some(rid("R2")));
    assert_eq!(d.suppressed_precedence_pairs, vec![(rid("R1"), rid("R2"))]);
    assert_eq!(d.inactive_deviation_ids, vec![rec("A1")]);
    assert_eq!(d.deviation_input, DeviationInput::Provided);
}

fn g26_policy() -> CompiledPolicy<InspectionOnly> {
    let mut p = ir(vec![rule("Rn", 10, Action::Block, not(card(1)))]);
    p.exclusions.push(e1(Some(105)));
    compiled(p)
}

#[test]
fn g26_exclusion_valid_at_decision_moment_blocks() {
    // Хост на момент t = 102 (CURRENT) передаёт E1 действительной.
    let d = decide(&g26_policy(), &docx_with_image(0), &valid(&["E1"]));
    assert_eq!(d.verdict, Verdict::Block);
    assert_eq!(d.winner_rule_id, Some(rid("Rn")));
    assert_eq!(d.part_exclusion_id, Some(rec("E1")));
    assert_eq!(d.deviation_input, DeviationInput::Provided);
}

#[test]
fn g27_exclusion_expired_at_decision_moment() {
    // t = 110 > expires_at: хост не передаёт E1 действительной.
    let d = decide(&g26_policy(), &docx_with_image(0), &valid(&[]));
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.reasons, vec![ReasonCode::UnsupportedFormat]);
    assert_eq!(d.fallback, Some(Fallback::AllowWithAudit));
    assert_eq!(d.required_action, Some(Action::Monitor));
    assert_eq!(d.inactive_deviation_ids, vec![rec("E1")]);
}

#[test]
fn g28_same_decision_moment_same_result() {
    // Дедлайн во вход PDP не входит: при одном моменте решения вход одинаков.
    let a = decide(&g26_policy(), &docx_with_image(0), &valid(&["E1"]));
    let b = decide(&g26_policy(), &docx_with_image(0), &valid(&["E1"]));
    assert_eq!(a, b);
    assert_eq!(a.verdict, Verdict::Block);
    assert_eq!(a.part_exclusion_id, Some(rec("E1")));
}

#[test]
fn g29_time_unverifiable_disables_exclusion() {
    let dv = DeviationValidity::Provided { time_state: TimeState::TimeUnverifiable, valid_ids: vec![] };
    let d = decide(&g26_policy(), &docx_with_image(0), &dv);
    assert_eq!(d.verdict, Verdict::Indeterminate);
    assert_eq!(d.reasons, vec![ReasonCode::UnsupportedFormat]);
    assert_eq!(d.fallback, Some(Fallback::AllowWithAudit));
    assert_eq!(d.required_action, Some(Action::Monitor));
    assert_eq!(d.inactive_deviation_ids, vec![rec("E1")]);
    assert_eq!(d.deviation_input, DeviationInput::Provided);
}

#[test]
fn g30_record_valid_at_decision_moment_applies() {
    let d = decide(&g26_policy(), &docx_with_image(0), &valid(&["E1"]));
    assert_eq!(d.verdict, Verdict::Block);
    assert_eq!(d.part_exclusion_id, Some(rec("E1")));
}
