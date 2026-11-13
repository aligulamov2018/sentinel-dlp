//! Тесты PDP этапа 1. Ожидания golden-тестов заданы явно по таблице
//! docs/architecture/SENTINEL_CORE_TEST_PLAN.md §5 и не вычисляются функциями алгоритма.
//! Свойства (oracle.rs) сверяются с независимым перебором, написанным отдельно от алгоритма.

mod contract;
mod golden;
mod oracle;

use crate::policy::{
    Approval, Atom, DetectorSpec, ExcludableKind, Expr, IncompleteAction, IncompleteInspectionAction,
    PartInspectionExclusion, PolicyIr, PrecedenceConflictApproval, Rule, Scope,
};
use crate::{
    BundleError, CompiledPolicy, InspectionOnly, ValidatedPolicy, ValidationParams, DEFAULT_WORLD_LIMIT,
};
use sentinel_core_types::*;

pub(crate) const T_VALIDATION: i64 = 100;

pub(crate) fn ctx() -> DecisionContext {
    DecisionContext {
        channel: Channel::new("USB"),
        operation: Operation::new("WRITE"),
        principal_ref: PrincipalRef::new("p1"),
        device_ref: DeviceRef::new("d1"),
        scope_attrs: vec![ScopeAttr::Group(GroupId::new("finance"))],
        policy_validity: PolicyValidity::Current,
        decision_time: None,
    }
}

pub(crate) fn ctx_group(g: &str) -> DecisionContext {
    DecisionContext { scope_attrs: vec![ScopeAttr::Group(GroupId::new(g))], ..ctx() }
}

pub(crate) fn det(id: &str, count: u32, met: bool) -> DetectionResult {
    DetectionResult {
        detector_id: DetectorId::new(id),
        detector_version: "1".into(),
        match_count: count,
        score: count,
        threshold_met: met,
    }
}

/// Результат инспекции с явно заданными полями.
#[allow(clippy::too_many_arguments)]
pub(crate) fn insp(
    status: InspectionStatus,
    reasons: &[ReasonCode],
    text: TextCoverage,
    structure: StructureEnumeration,
    classification: Classification,
    parts: &[PartKind],
    detections: Vec<DetectionResult>,
    class: &str,
) -> InspectionResult {
    let mut detections = detections;
    detections.sort_by(|a, b| a.detector_id.cmp(&b.detector_id));
    InspectionResult {
        status,
        reasons: reasons.to_vec(),
        text_coverage: text,
        structure_enumeration: structure,
        classification,
        uninspected_parts: parts.iter().map(|k| UninspectedPart { kind: *k, count: 1 }).collect(),
        detections,
        file_class: FileClass::new(class),
        digest: ContentDigest::not_computed(DigestStatus::NotRequested),
        engine: EngineInfo {
            core_version: "0.1.0".into(),
            detector_set_version: "ds-1".into(),
            limits_profile_id: "lp-1".into(),
        },
        inspection_origin: InspectionOrigin::Core,
    }
}

/// Ничего не разобрано: `NOT_INSPECTED(reason)`.
pub(crate) fn not_inspected(reason: ReasonCode) -> InspectionResult {
    insp(
        InspectionStatus::NotInspected,
        &[reason],
        TextCoverage::Partial,
        StructureEnumeration::Incomplete(reason),
        Classification::Confirmed,
        &[],
        vec![],
        "OOXML",
    )
}

/// Текст разобран частично до лимита или дедлайна; найдено `found` номеров карт.
pub(crate) fn partial_text(reason: ReasonCode, found: u32) -> InspectionResult {
    let status = if found >= 1 { InspectionStatus::Match } else { InspectionStatus::NotInspected };
    insp(
        status,
        &[reason],
        TextCoverage::Partial,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[],
        vec![det("card", found, found >= 1)],
        "TXT",
    )
}

/// DOCX: текст и перечисление полные, одно изображение.
pub(crate) fn docx_with_image(found: u32) -> InspectionResult {
    let status = if found >= 1 { InspectionStatus::Match } else { InspectionStatus::NotInspected };
    insp(
        status,
        &[ReasonCode::UnsupportedFormat],
        TextCoverage::Complete,
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[PartKind::Image],
        vec![det("card", found, found >= 1)],
        "OOXML",
    )
}

pub(crate) fn card(n: u32) -> Expr {
    Expr::Atom(Atom::CountAtLeast(DetectorId::new("card"), n))
}
pub(crate) fn count(d: &str, n: u32) -> Expr {
    Expr::Atom(Atom::CountAtLeast(DetectorId::new(d), n))
}
pub(crate) fn usb() -> Expr {
    Expr::Atom(Atom::ChannelIs(Channel::new("USB")))
}
pub(crate) fn incomplete(r: ReasonCode) -> Expr {
    Expr::Atom(Atom::InspectionIncomplete(Some(r)))
}
pub(crate) fn class_is(c: &str) -> Expr {
    Expr::Atom(Atom::FileClassIs(FileClass::new(c)))
}
pub(crate) fn not(e: Expr) -> Expr {
    Expr::Not(Box::new(e))
}

pub(crate) fn rule(id: &str, priority: u32, action: Action, condition: Expr) -> Rule {
    Rule {
        rule_id: RuleId::new(id),
        priority,
        protected_operation: false,
        scope: Scope::default(),
        condition,
        action,
        requested_action: None,
        capability_exception_id: None,
        on_indeterminate: Fallback::AllowWithAudit,
        data_level: DataLevel::Metadata,
    }
}

pub(crate) fn on_ind_block(mut r: Rule) -> Rule {
    r.on_indeterminate = Fallback::Block;
    r.protected_operation = true;
    r
}

pub(crate) fn finance() -> Scope {
    Scope { groups: vec![GroupId::new("finance")], ..Scope::default() }
}

pub(crate) fn approval_meta(expires_at: Option<i64>) -> Approval {
    Approval {
        justification: "обоснование".into(),
        author: "author".into(),
        approver: "approver".into(),
        approved_at: 50,
        expires_at,
        max_offline_validity: 3600,
    }
}

pub(crate) fn e1(expires_at: Option<i64>) -> PartInspectionExclusion {
    PartInspectionExclusion {
        exclusion_id: RecordId::new("E1"),
        scope: finance(),
        kinds: vec![ExcludableKind::Image],
        approval: approval_meta(expires_at),
    }
}

pub(crate) fn pca(id: &str, h: &str, l: &str) -> PrecedenceConflictApproval {
    PrecedenceConflictApproval {
        approval_id: RecordId::new(id),
        higher_rule_id: RuleId::new(h),
        lower_rule_id: RuleId::new(l),
        scope: finance(),
        approval: approval_meta(None),
    }
}

pub(crate) fn iia(id: &str, scope: Scope, action: IncompleteAction) -> IncompleteInspectionAction {
    IncompleteInspectionAction { record_id: RecordId::new(id), scope, action }
}

/// IR с детекторами card, kw, sensitive (порог 1) и записью контракта MONITOR без ограничения.
pub(crate) fn ir(rules: Vec<Rule>) -> PolicyIr {
    PolicyIr {
        bundle_id: BundleId::new("bundle-1"),
        policy_version: 42,
        format_version: (1, 0),
        platform: "windows".into(),
        detector_set: ["card", "kw", "sensitive"]
            .iter()
            .map(|d| DetectorSpec { detector_id: DetectorId::new(*d), threshold: 1 })
            .collect(),
        rules,
        incomplete_actions: vec![iia("IIA-default", Scope::default(), IncompleteAction::Monitor)],
        exclusions: vec![],
        approvals: vec![],
    }
}

pub(crate) fn params() -> ValidationParams {
    ValidationParams { world_limit: DEFAULT_WORLD_LIMIT, validation_time: T_VALIDATION }
}

pub(crate) fn validate(ir: PolicyIr) -> Result<ValidatedPolicy, BundleError> {
    ValidatedPolicy::validate(ir, &params())
}

pub(crate) fn compiled(ir: PolicyIr) -> CompiledPolicy<InspectionOnly> {
    CompiledPolicy::unsigned_for_tests(validate(ir).expect("политика должна пройти проверку"))
}

pub(crate) fn valid(ids: &[&str]) -> DeviationValidity {
    DeviationValidity::Provided {
        time_state: TimeState::Current,
        valid_ids: ids.iter().map(|i| RecordId::new(*i)).collect(),
    }
}

pub(crate) fn rid(s: &str) -> RuleId {
    RuleId::new(s)
}
pub(crate) fn rec(s: &str) -> RecordId {
    RecordId::new(s)
}

pub(crate) fn decide(
    p: &CompiledPolicy<InspectionOnly>,
    i: &InspectionResult,
    dv: &DeviationValidity,
) -> Decision {
    crate::Pdp::decide(p, &ctx(), i, dv).expect("решение должно быть получено")
}
