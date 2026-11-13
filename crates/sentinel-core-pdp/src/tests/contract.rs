//! Контрактные и негативные тесты: вход `DeviationValidity` (§6.8, TC-CORE-45 в части PDP),
//! структурная проверка бандла (§6.7; TC-CORE-28, -33, -35 в части загрузки), состояние
//! политики (TC-CORE-17), решение без правила (TC-CORE-30), ошибки вызова, детерминизм.

use super::*;
use crate::policy::{ConflictClass, IncompleteAction};
use crate::{BundleError, EngineFault, InvalidRequest, Pdp, PolicyState, PolicyStateKind};

fn g06() -> CompiledPolicy<InspectionOnly> {
    let mut p = ir(vec![rule("Rc", 10, Action::Block, card(1))]);
    p.exclusions.push(e1(None));
    p.rules.push(rule("Rz", 1, Action::Monitor, usb()));
    compiled(p)
}

// ---------- §6.8: отсутствующий и противоречивый вход ----------

#[test]
fn deviation_unavailable_disables_all() {
    let d = decide(&g06(), &docx_with_image(0), &DeviationValidity::Unavailable);
    assert_eq!(d.deviation_input, DeviationInput::Unavailable);
    assert_eq!(d.part_exclusion_id, None);
    assert_eq!(d.inactive_deviation_ids, vec![rec("E1")]);
}

#[test]
fn deviation_time_unverifiable_with_ids_is_contradictory() {
    let dv =
        DeviationValidity::Provided { time_state: TimeState::TimeUnverifiable, valid_ids: vec![rec("E1")] };
    let d = decide(&g06(), &docx_with_image(0), &dv);
    assert_eq!(d.deviation_input, DeviationInput::Contradictory);
    assert_eq!(d.part_exclusion_id, None);
    assert_eq!(d.explanation.deviation_contradictions.len(), 1);
}

#[test]
fn deviation_unknown_id_invalidates_all_not_only_disputed() {
    let d = decide(&g06(), &docx_with_image(0), &valid(&["E1", "NO-SUCH"]));
    assert_eq!(d.deviation_input, DeviationInput::Contradictory);
    assert_eq!(d.part_exclusion_id, None, "E1 тоже недействительна при противоречивом входе");
}

#[test]
fn deviation_non_deviation_record_is_contradictory() {
    let d = decide(&g06(), &docx_with_image(0), &valid(&["IIA-default"]));
    assert_eq!(d.deviation_input, DeviationInput::Contradictory);
}

#[test]
fn deviation_duplicates_are_normalized() {
    let d = decide(&g06(), &docx_with_image(0), &valid(&["E1", "E1"]));
    assert_eq!(d.deviation_input, DeviationInput::Provided);
    assert_eq!(d.part_exclusion_id, Some(rec("E1")));
}

#[test]
fn deviation_input_never_causes_engine_fault() {
    for dv in [DeviationValidity::Unavailable, valid(&["X"]), valid(&[])] {
        assert!(Pdp::decide(&g06(), &ctx(), &docx_with_image(0), &dv).is_ok());
    }
}

#[test]
fn exclusion_outside_scope_is_not_applied() {
    // Действительная запись вне области: не применяется и не считается недействительной.
    let d = Pdp::decide(&g06(), &ctx_group("other"), &docx_with_image(0), &valid(&["E1"])).unwrap();
    assert_eq!(d.part_exclusion_id, None);
    assert!(d.inactive_deviation_ids.is_empty());
    // Недействительная запись перечисляется независимо от области (SENTINEL_CORE_DESIGN.md §5).
    let d = Pdp::decide(&g06(), &ctx_group("other"), &docx_with_image(0), &valid(&[])).unwrap();
    assert_eq!(d.inactive_deviation_ids, vec![rec("E1")]);
}

// ---------- TC-CORE-28: условия исключения анализа частей ----------

fn with_parts(parts: &[PartKind]) -> InspectionResult {
    insp(
        InspectionStatus::NotInspected,
        &[ReasonCode::UnsupportedFormat],
        TextCoverage::Complete,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        parts,
        vec![det("card", 0, false)],
        "OOXML",
    )
}

#[test]
fn exclusion_not_applied_for_unknown_or_foreign_kind() {
    for parts in [&[PartKind::Unknown][..], &[PartKind::Image, PartKind::Attachment][..]] {
        let d = decide(&g06(), &with_parts(parts), &valid(&["E1"]));
        assert_eq!(d.part_exclusion_id, None, "части {parts:?}");
    }
}

#[test]
fn exclusion_not_applied_for_partial_text() {
    let i = insp(
        InspectionStatus::NotInspected,
        &[ReasonCode::ResourceExhausted, ReasonCode::UnsupportedFormat],
        TextCoverage::Partial,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[PartKind::Image],
        vec![det("card", 0, false)],
        "OOXML",
    );
    assert_eq!(decide(&g06(), &i, &valid(&["E1"])).part_exclusion_id, None);
}

#[test]
fn lowest_exclusion_id_wins() {
    let mut p = ir(vec![rule("Rc", 10, Action::Block, card(1))]);
    let mut e0 = e1(None);
    e0.exclusion_id = rec("E0");
    p.exclusions = vec![e1(None), e0];
    let d = decide(&compiled(p), &docx_with_image(0), &valid(&["E1", "E0"]));
    assert_eq!(d.part_exclusion_id, Some(rec("E0")));
}

// ---------- §6.7: структурная проверка ----------

#[test]
fn missing_unbounded_incomplete_action_rejected() {
    let mut p = ir(vec![]);
    p.incomplete_actions = vec![iia(
        "IIA-usb",
        Scope { channels: vec![Channel::new("USB")], ..Scope::default() },
        IncompleteAction::Alert,
    )];
    assert_eq!(validate(p).unwrap_err(), BundleError::MissingUnboundedIncompleteAction);
}

#[test]
fn exclusion_checks() {
    let mut p = ir(vec![]);
    let mut e = e1(None);
    e.scope = Scope { channels: vec![Channel::new("USB")], ..Scope::default() };
    p.exclusions = vec![e];
    assert_eq!(validate(p).unwrap_err(), BundleError::UnboundedDeviationScope(rec("E1")));

    let mut p = ir(vec![]);
    let mut e = e1(None);
    e.kinds.clear();
    p.exclusions = vec![e];
    assert_eq!(validate(p).unwrap_err(), BundleError::EmptyExclusionKinds(rec("E1")));

    let mut p = ir(vec![]);
    let mut e = e1(None);
    e.approval.approver = e.approval.author.clone();
    p.exclusions = vec![e];
    assert_eq!(validate(p).unwrap_err(), BundleError::ApproverIsAuthor(rec("E1")));

    let mut p = ir(vec![]);
    let mut e = e1(None);
    e.approval.justification = "  ".into();
    p.exclusions = vec![e];
    assert_eq!(validate(p).unwrap_err(), BundleError::MissingJustification(rec("E1")));

    let mut p = ir(vec![]);
    p.exclusions = vec![e1(Some(T_VALIDATION))];
    assert_eq!(validate(p).unwrap_err(), BundleError::RecordExpired(rec("E1")));
}

#[test]
fn approval_scope_must_be_bounded_and_within_pair() {
    let rules = || {
        let mut h = rule("R1", 100, Action::Alert, usb());
        h.scope = finance();
        vec![h, rule("R2", 10, Action::Block, count("sensitive", 1))]
    };
    let mut p = ir(rules());
    let mut a = pca("A1", "R1", "R2");
    a.scope = Scope { groups: vec![GroupId::new("hr")], ..Scope::default() };
    p.approvals = vec![a];
    assert_eq!(validate(p).unwrap_err(), BundleError::ApprovalScopeOutsidePair(rec("A1")));

    let mut p = ir(rules());
    let mut a = pca("A1", "R1", "R2");
    a.scope = Scope::default();
    p.approvals = vec![a];
    assert_eq!(validate(p).unwrap_err(), BundleError::UnboundedDeviationScope(rec("A1")));

    let mut p = ir(rules());
    p.approvals = vec![pca("A1", "R1", "R2")];
    assert!(validate(p).is_ok());
}

#[test]
fn provably_disjoint_scopes_need_no_approval() {
    // Однозначные измерения контекста: устройства и каналы.
    let mut h = rule("R1", 100, Action::Alert, usb());
    h.scope = Scope { devices: vec![DeviceRef::new("d1")], ..Scope::default() };
    let mut l = rule("R2", 10, Action::Block, count("sensitive", 1));
    l.scope = Scope { devices: vec![DeviceRef::new("d2")], ..Scope::default() };
    assert!(validate(ir(vec![h, l])).is_ok());
    let mut h = rule("R1", 100, Action::Alert, card(1));
    h.scope = Scope { channels: vec![Channel::new("PRINT")], ..Scope::default() };
    let mut l = rule("R2", 10, Action::Block, count("sensitive", 1));
    l.scope = Scope { channels: vec![Channel::new("USB")], ..Scope::default() };
    assert!(validate(ir(vec![h, l])).is_ok());
    // Разные константы context.channel среди конъюнктов верхнего уровня.
    let h = rule("R1", 100, Action::Alert, Expr::Atom(crate::policy::Atom::ChannelIs(Channel::new("PRINT"))));
    let l = rule("R2", 10, Action::Block, Expr::And(vec![usb(), count("sensitive", 1)]));
    assert!(validate(ir(vec![h, l])).is_ok());
}

/// CHG7-R1: группы и подразделения многозначны, поэтому разные группы областей не доказывают
/// непересечение — субъект может состоять в обеих.
#[test]
fn chg7_r1_multi_valued_groups_do_not_prove_disjointness() {
    let hr = || Scope { groups: vec![GroupId::new("hr")], ..Scope::default() };
    let both = || Scope { groups: vec![GroupId::new("hr"), GroupId::new("finance")], ..Scope::default() };
    let ctx_hr_finance = DecisionContext {
        scope_attrs: vec![ScopeAttr::Group(GroupId::new("hr")), ScopeAttr::Group(GroupId::new("finance"))],
        ..ctx()
    };
    // Контрпример: h — ALERT для hr, l — BLOCK для finance; субъект в обеих группах видит оба правила.
    let mut h = rule("R1", 100, Action::Alert, usb());
    h.scope = hr();
    let mut l = rule("R2", 10, Action::Block, usb());
    l.scope = finance();
    assert!(h.scope.matches(&ctx_hr_finance) && l.scope.matches(&ctx_hr_finance));
    assert!(!crate::policy::rules_provably_disjoint(&h, &l));
    // Без утверждения бандл отклоняется.
    assert_eq!(
        validate(ir(vec![h.clone(), l.clone()])).unwrap_err(),
        BundleError::UnapprovedPrecedenceConflict {
            higher: rid("R1"),
            lower: rid("R2"),
            class: ConflictClass::C
        }
    );
    // Ограниченное утверждение для пары hr / finance невыразимо: пересечение групп пусто.
    for groups in [
        vec![GroupId::new("hr")],
        vec![GroupId::new("finance")],
        vec![GroupId::new("hr"), GroupId::new("finance")],
    ] {
        let mut p = ir(vec![h.clone(), l.clone()]);
        let mut a = pca("A1", "R1", "R2");
        a.scope = Scope { groups, ..Scope::default() };
        p.approvals = vec![a];
        assert_eq!(validate(p).unwrap_err(), BundleError::ApprovalScopeOutsidePair(rec("A1")));
    }
    // То же для подразделений.
    let mut h2 = h.clone();
    h2.scope = Scope { org_units: vec![OrgUnitId::new("ou-a")], ..Scope::default() };
    let mut l2 = l.clone();
    l2.scope = Scope { org_units: vec![OrgUnitId::new("ou-b")], ..Scope::default() };
    assert!(matches!(
        validate(ir(vec![h2, l2])).unwrap_err(),
        BundleError::UnapprovedPrecedenceConflict { .. }
    ));

    // Выразимый случай: l — BLOCK для hr и finance, h — ALERT для hr; утверждение для hr лежит в пересечении.
    let mut l3 = l.clone();
    l3.scope = both();
    let mut p = ir(vec![h.clone(), l3]);
    let mut a = pca("A1", "R1", "R2");
    a.scope = hr();
    p.approvals = vec![a];
    let c = compiled(p);
    let full_txt = insp(
        InspectionStatus::NoMatch,
        &[],
        TextCoverage::Complete,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[],
        vec![],
        "TXT",
    );
    // Субъект в обеих группах, утверждение действительно → ALERT (R1) с утверждением в решении.
    let d = Pdp::decide(&c, &ctx_hr_finance, &full_txt, &valid(&["A1"])).unwrap();
    assert_eq!(
        (d.verdict, d.winner_rule_id.clone(), d.required_action),
        (Verdict::Allow, Some(rid("R1")), Some(Action::Alert))
    );
    assert_eq!(d.precedence_approval_ids, vec![rec("A1")]);
    // Утверждение недействительно → пара подавлена, BLOCK (R2).
    let d = Pdp::decide(&c, &ctx_hr_finance, &full_txt, &valid(&[])).unwrap();
    assert_eq!((d.verdict, d.winner_rule_id.clone()), (Verdict::Block, Some(rid("R2"))));
    assert_eq!(d.suppressed_precedence_pairs, vec![(rid("R1"), rid("R2"))]);
    // Субъект только в finance: R1 неприменимо, конфликта нет, BLOCK.
    let d = Pdp::decide(&c, &ctx_group("finance"), &full_txt, &valid(&[])).unwrap();
    assert_eq!((d.verdict, d.winner_rule_id), (Verdict::Block, Some(rid("R2"))));
    assert!(d.suppressed_precedence_pairs.is_empty());
}

#[test]
fn incompleteness_atom_only_top_level() {
    let bad = rule("R1", 10, Action::Block, Expr::Or(vec![incomplete(ReasonCode::Timeout), usb()]));
    assert_eq!(validate(ir(vec![bad])).unwrap_err(), BundleError::IncompletenessAtomNotTopLevel(rid("R1")));
    let bad = rule("R1", 10, Action::Block, not(incomplete(ReasonCode::Timeout)));
    assert_eq!(validate(ir(vec![bad])).unwrap_err(), BundleError::IncompletenessAtomNotTopLevel(rid("R1")));
    let ok = rule("R1", 10, Action::Block, Expr::And(vec![incomplete(ReasonCode::Timeout), usb()]));
    assert!(validate(ir(vec![ok])).is_ok());
}

#[test]
fn rules_must_be_ordered_and_unique() {
    let err = validate(ir(vec![rule("R2", 5, Action::Block, usb()), rule("R1", 10, Action::Block, usb())]))
        .unwrap_err();
    assert_eq!(err, BundleError::RulesNotOrdered { previous: rid("R2"), next: rid("R1") });
    // Равный приоритет: более строгое действие выше.
    let err = validate(ir(vec![rule("R1", 5, Action::Alert, card(1)), rule("R2", 5, Action::Block, usb())]))
        .unwrap_err();
    assert_eq!(err, BundleError::RulesNotOrdered { previous: rid("R1"), next: rid("R2") });
    // Равные приоритет и строгость: меньший rule_id выше.
    let err = validate(ir(vec![rule("R2", 5, Action::Block, usb()), rule("R1", 5, Action::Block, usb())]))
        .unwrap_err();
    assert_eq!(err, BundleError::RulesNotOrdered { previous: rid("R2"), next: rid("R1") });
    let err = validate(ir(vec![rule("R1", 5, Action::Block, usb()), rule("R1", 4, Action::Block, usb())]))
        .unwrap_err();
    assert_eq!(err, BundleError::DuplicateRuleId(rid("R1")));
}

#[test]
fn unknown_detector_and_format_version() {
    let err = validate(ir(vec![rule("R1", 5, Action::Block, count("nope", 1))])).unwrap_err();
    assert!(matches!(err, BundleError::UnknownDetector { .. }));
    let mut p = ir(vec![]);
    p.format_version = (2, 0);
    assert_eq!(validate(p).unwrap_err(), BundleError::UnsupportedFormatVersion { major: 2 });
}

#[test]
fn complexity_limit_enforced() {
    let rules: Vec<Rule> =
        (0..13u32).map(|i| rule(&format!("R{i:02}"), 100 - i, Action::Block, card(i + 1))).collect();
    let p = ir(rules);
    let tight = ValidationParams { world_limit: 13, validation_time: T_VALIDATION };
    assert_eq!(
        ValidatedPolicy::validate(p.clone(), &tight).unwrap_err(),
        BundleError::ComplexityExceeded { worlds: 14, limit: 13 }
    );
    assert!(validate(p).is_ok());
}

#[test]
fn conflict_class_reported() {
    let err = validate(ir(vec![rule("H", 10, Action::Monitor, usb()), rule("L", 5, Action::Block, usb())]))
        .unwrap_err();
    assert_eq!(
        err,
        BundleError::UnapprovedPrecedenceConflict {
            higher: rid("H"),
            lower: rid("L"),
            class: ConflictClass::C
        }
    );
}

// ---------- TC-CORE-17: состояние политики ----------

#[test]
fn policy_state_absent_invalid_expired() {
    for (kind, reason) in [
        (PolicyStateKind::Absent, ReasonCode::PolicyAbsent),
        (PolicyStateKind::Invalid, ReasonCode::PolicyInvalid),
        (PolicyStateKind::Expired, ReasonCode::PolicyExpired),
    ] {
        let st = PolicyState { kind, bootstrap_fallback: Fallback::AllowWithAudit };
        let d = Pdp::decide_without_policy(&st, &ctx(), &not_inspected(ReasonCode::Timeout), &valid(&[]))
            .unwrap();
        assert_eq!(d.verdict, Verdict::Indeterminate);
        assert_eq!(d.reasons, vec![reason]);
        assert_eq!(d.fallback, Some(Fallback::AllowWithAudit));
        assert_eq!(d.required_action, Some(Action::Monitor));
        assert_eq!(d.policy_bundle_id, None);
        assert_eq!(d.verification_mode, None);
    }
    let st = PolicyState { kind: PolicyStateKind::Absent, bootstrap_fallback: Fallback::Block };
    let d = Pdp::decide_without_policy(&st, &ctx(), &not_inspected(ReasonCode::Timeout), &valid(&["E1"]))
        .unwrap();
    assert_eq!(d.required_action, Some(Action::Block));
    assert_eq!(d.deviation_input, DeviationInput::Contradictory);
}

#[test]
fn expired_with_compiled_policy_is_invalid_request() {
    let c = DecisionContext { policy_validity: PolicyValidity::Expired, ..ctx() };
    let r = Pdp::decide(&g06(), &c, &docx_with_image(0), &valid(&[]));
    assert_eq!(r.unwrap_err(), EngineFault::InvalidRequest(InvalidRequest::ExpiredPolicyPassedAsCompiled));
}

#[test]
fn stale_policy_marks_decision_degraded() {
    let c = DecisionContext { policy_validity: PolicyValidity::Stale, ..ctx() };
    let d = Pdp::decide(&g06(), &c, &docx_with_image(0), &valid(&[])).unwrap();
    assert!(d.degraded);
    assert!(!decide(&g06(), &docx_with_image(0), &valid(&[])).degraded);
}

// ---------- ошибки вызова ----------

#[test]
fn inspection_contract_violations_rejected() {
    let mut i = docx_with_image(0);
    i.status = InspectionStatus::NoMatch; // NO_MATCH при неполном покрытии (INV-C4)
    assert!(matches!(
        Pdp::decide(&g06(), &ctx(), &i, &valid(&[])),
        Err(EngineFault::InvalidRequest(InvalidRequest::InspectionContract(_)))
    ));
    let mut i = docx_with_image(0);
    i.reasons.clear(); // неполнота без причин
    assert!(Pdp::decide(&g06(), &ctx(), &i, &valid(&[])).is_err());
    let mut i = docx_with_image(0);
    i.reasons = vec![ReasonCode::PolicyAbsent];
    assert!(Pdp::decide(&g06(), &ctx(), &i, &valid(&[])).is_err());
}

// ---------- TC-CORE-30: решение без выбранного правила ----------

#[test]
fn no_rule_full_vs_incomplete() {
    let p = compiled(ir(vec![rule("Rc", 10, Action::Block, card(1))]));
    let full = insp(
        InspectionStatus::NoMatch,
        &[],
        TextCoverage::Complete,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[],
        vec![],
        "TXT",
    );
    let d = decide(&p, &full, &valid(&[]));
    assert_eq!((d.decision_basis, d.required_action), (DecisionBasis::NoRuleMatched, None));
    let p = compiled(ir(vec![rule(
        "Rc",
        10,
        Action::Block,
        Expr::And(vec![card(1), Expr::Atom(crate::policy::Atom::ChannelIs(Channel::new("PRINT")))]),
    )]));
    let d = decide(&p, &not_inspected(ReasonCode::Timeout), &valid(&[]));
    assert_eq!(d.decision_basis, DecisionBasis::OutcomeIndependentOfUnknown);
    assert_eq!(d.required_action, Some(Action::Monitor));
}

// ---------- INV-C7 / TC-CORE-15: детерминизм ----------

#[test]
fn decisions_are_deterministic() {
    let p = g06();
    let i = docx_with_image(0);
    let first = decide(&p, &i, &valid(&["E1"]));
    for _ in 0..20 {
        assert_eq!(decide(&p, &i, &valid(&["E1"])), first);
    }
}

#[test]
fn explanation_carries_no_match_values_and_names_atoms() {
    let d = decide(&g06(), &docx_with_image(0), &valid(&[]));
    let rc = d.explanation.rules.iter().find(|r| r.rule_id == rid("Rc")).unwrap();
    assert_eq!(rc.value, Truth::Unknown);
    assert_eq!(rc.determining_atom.as_deref(), Some("detector(card).count >= 1"));
    assert_eq!(d.explanation.detectors, vec![(DetectorId::new("card"), 0, false)]);
}

/// CHG7-R2: предварительное решение даёт только сводку; `Decision` из него не извлекается
/// (проверки типов — compile_fail-доктесты `ProvisionalDecision` в sentinel-core-types).
#[test]
fn chg7_r2_provisional_decision_exposes_only_summary() {
    let d = decide(&g06(), &docx_with_image(0), &valid(&[]));
    let p = ProvisionalDecision::new(d.clone());
    let s = p.summary();
    assert_eq!(s.verdict, d.verdict);
    assert_eq!(s.reasons, d.reasons);
    assert_eq!(s.decision_basis, d.decision_basis);
    assert_eq!(s.possible_winners, d.possible_winners);
    assert_eq!(s.deviation_input, d.deviation_input);
    // Debug не раскрывает полного решения (объяснение, источники действий, уровень данных).
    let dbg = format!("{p:?}");
    assert!(dbg.starts_with("ProvisionalDecision { summary: ProvisionalSummary {"), "{dbg}");
    for hidden in ["explanation", "action_sources", "data_level", "policy_bundle_id"] {
        assert!(!dbg.contains(hidden), "в Debug попало поле {hidden}: {dbg}");
    }
}
