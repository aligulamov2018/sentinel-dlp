//! Структурная проверка IR при загрузке (SENTINEL_CORE_DESIGN.md §6.7).
//!
//! Реализованы строки таблицы §6.7, которые не требуют подписи и манифеста возможностей:
//! версия формата, состав, контракт неполной инспекции, исключения анализа частей, конфликт
//! приоритетов, сложность. Не реализованы на этапе 1: подпись и якоря доверия (O-05),
//! «версия не ниже текущей» (нужно хранилище состояния хоста), строки «Манифест»,
//! «Платформа», «BLOCK», «Понижение», «`on_indeterminate = BLOCK`», «UNSUPPORTED» — они
//! относятся к режиму `EnforcementReady` и требуют проверенного манифеста.

use crate::policy::{
    conflict_pairs, precedence, Approval, Atom, ConflictClass, Expr, PolicyIr, Rule, SUPPORTED_FORMAT_MAJOR,
};
use sentinel_core_types::{DetectorId, RecordId, RuleId};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

/// Предел числа миров по умолчанию — Proposed (§6.7, «Сложность»; вопрос O-07).
pub const DEFAULT_WORLD_LIMIT: u128 = 4096;

/// Параметры проверки, которые задаёт хост.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidationParams {
    /// Предел числа миров профиля (Proposed: 4096).
    pub world_limit: u128,
    /// Момент проверки срока записей-отступлений при загрузке (Unix-секунды). Ядро часов не
    /// читает; значение передаёт хост по своей оценке времени.
    pub validation_time: i64,
}

/// Ошибка загрузки бандла: бандл не принимается, действует прежний (ADR-0016, решение 7).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BundleError {
    UnsupportedFormatVersion { major: u32 },
    DuplicateRuleId(RuleId),
    DuplicateRecordId(RecordId),
    DuplicateDetectorId(DetectorId),
    RulesNotOrdered { previous: RuleId, next: RuleId },
    EmptyCompositeCondition(RuleId),
    UnknownDetector { rule: RuleId, detector: DetectorId },
    IncompletenessAtomNotTopLevel(RuleId),
    MissingUnboundedIncompleteAction,
    UnboundedDeviationScope(RecordId),
    EmptyExclusionKinds(RecordId),
    MissingJustification(RecordId),
    MissingAuthorOrApprover(RecordId),
    ApproverIsAuthor(RecordId),
    RecordExpired(RecordId),
    ApprovalUnknownRule { approval: RecordId, rule: RuleId },
    ApprovalScopeOutsidePair(RecordId),
    UnapprovedPrecedenceConflict { higher: RuleId, lower: RuleId, class: ConflictClass },
    ComplexityExceeded { worlds: u128, limit: u128 },
}

/// IR, прошедшее структурную проверку §6.7.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedPolicy {
    ir: PolicyIr,
    world_bound: u128,
}

impl ValidatedPolicy {
    /// Проверяет IR. Любое нарушение — `BundleError`.
    pub fn validate(ir: PolicyIr, params: &ValidationParams) -> Result<Self, BundleError> {
        if ir.format_version.0 != SUPPORTED_FORMAT_MAJOR {
            return Err(BundleError::UnsupportedFormatVersion { major: ir.format_version.0 });
        }
        check_unique_ids(&ir)?;
        for pair in ir.rules.windows(2) {
            if precedence(&pair[0], &pair[1]) != Ordering::Less {
                return Err(BundleError::RulesNotOrdered {
                    previous: pair[0].rule_id.clone(),
                    next: pair[1].rule_id.clone(),
                });
            }
        }
        let detectors: BTreeSet<&DetectorId> = ir.detector_set.iter().map(|d| &d.detector_id).collect();
        for rule in &ir.rules {
            check_condition(rule, &detectors)?;
        }
        if !ir.incomplete_actions.iter().any(|r| r.scope.is_unbounded()) {
            return Err(BundleError::MissingUnboundedIncompleteAction);
        }
        for ex in &ir.exclusions {
            if !ex.scope.is_bounded_subject() {
                return Err(BundleError::UnboundedDeviationScope(ex.exclusion_id.clone()));
            }
            if ex.kinds.is_empty() {
                return Err(BundleError::EmptyExclusionKinds(ex.exclusion_id.clone()));
            }
            check_approval(&ex.exclusion_id, &ex.approval, params)?;
        }
        let rule_by_id: BTreeMap<&RuleId, &Rule> = ir.rules.iter().map(|r| (&r.rule_id, r)).collect();
        for ap in &ir.approvals {
            check_approval(&ap.approval_id, &ap.approval, params)?;
            let h = rule_by_id.get(&ap.higher_rule_id).ok_or_else(|| BundleError::ApprovalUnknownRule {
                approval: ap.approval_id.clone(),
                rule: ap.higher_rule_id.clone(),
            })?;
            let l = rule_by_id.get(&ap.lower_rule_id).ok_or_else(|| BundleError::ApprovalUnknownRule {
                approval: ap.approval_id.clone(),
                rule: ap.lower_rule_id.clone(),
            })?;
            if !ap.scope.is_bounded_subject() {
                return Err(BundleError::UnboundedDeviationScope(ap.approval_id.clone()));
            }
            if !ap.scope.within_intersection(&h.scope, &l.scope) {
                return Err(BundleError::ApprovalScopeOutsidePair(ap.approval_id.clone()));
            }
        }
        let ordered: Vec<&Rule> = ir.rules.iter().collect();
        for pair in conflict_pairs(&ordered) {
            let (h, l) = (ordered[pair.higher], ordered[pair.lower]);
            let approved =
                ir.approvals.iter().any(|a| a.higher_rule_id == h.rule_id && a.lower_rule_id == l.rule_id);
            if !approved {
                return Err(BundleError::UnapprovedPrecedenceConflict {
                    higher: h.rule_id.clone(),
                    lower: l.rule_id.clone(),
                    class: pair.class,
                });
            }
        }
        let world_bound = world_bound(&ir);
        if world_bound > params.world_limit {
            return Err(BundleError::ComplexityExceeded { worlds: world_bound, limit: params.world_limit });
        }
        Ok(Self { ir, world_bound })
    }

    /// Проверенное IR.
    pub fn ir(&self) -> &PolicyIr {
        &self.ir
    }

    /// Верхняя граница числа миров для любого объекта и контекста (§6.7).
    pub fn world_bound(&self) -> u128 {
        self.world_bound
    }
}

fn check_unique_ids(ir: &PolicyIr) -> Result<(), BundleError> {
    let mut rules = BTreeSet::new();
    for r in &ir.rules {
        if !rules.insert(&r.rule_id) {
            return Err(BundleError::DuplicateRuleId(r.rule_id.clone()));
        }
    }
    let mut dets = BTreeSet::new();
    for d in &ir.detector_set {
        if !dets.insert(&d.detector_id) {
            return Err(BundleError::DuplicateDetectorId(d.detector_id.clone()));
        }
    }
    let mut records = BTreeSet::new();
    let ids = ir
        .incomplete_actions
        .iter()
        .map(|r| &r.record_id)
        .chain(ir.exclusions.iter().map(|e| &e.exclusion_id))
        .chain(ir.approvals.iter().map(|a| &a.approval_id));
    for id in ids {
        if !records.insert(id) {
            return Err(BundleError::DuplicateRecordId(id.clone()));
        }
    }
    Ok(())
}

fn check_condition(rule: &Rule, detectors: &BTreeSet<&DetectorId>) -> Result<(), BundleError> {
    fn walk(e: &Expr, rule: &Rule, detectors: &BTreeSet<&DetectorId>, top: bool) -> Result<(), BundleError> {
        match e {
            Expr::Atom(a) => {
                if let Some(d) = a.detector() {
                    if !detectors.contains(d) {
                        return Err(BundleError::UnknownDetector {
                            rule: rule.rule_id.clone(),
                            detector: d.clone(),
                        });
                    }
                }
                if matches!(a, Atom::InspectionIncomplete(_)) && !top {
                    return Err(BundleError::IncompletenessAtomNotTopLevel(rule.rule_id.clone()));
                }
                Ok(())
            }
            Expr::And(xs) | Expr::Or(xs) if xs.is_empty() => {
                Err(BundleError::EmptyCompositeCondition(rule.rule_id.clone()))
            }
            Expr::And(xs) => xs.iter().try_for_each(|x| walk(x, rule, detectors, false)),
            Expr::Or(xs) => xs.iter().try_for_each(|x| walk(x, rule, detectors, false)),
            Expr::Not(x) => walk(x, rule, detectors, false),
        }
    }
    // Конъюнкты верхнего уровня: атом неполноты допустим только здесь (§6.1).
    match &rule.condition {
        Expr::And(xs) if xs.is_empty() => Err(BundleError::EmptyCompositeCondition(rule.rule_id.clone())),
        Expr::And(xs) => xs.iter().try_for_each(|x| match x {
            Expr::Atom(_) => walk(x, rule, detectors, true),
            other => walk(other, rule, detectors, false),
        }),
        other => walk(other, rule, detectors, true),
    }
}

fn check_approval(id: &RecordId, a: &Approval, params: &ValidationParams) -> Result<(), BundleError> {
    if a.justification.trim().is_empty() {
        return Err(BundleError::MissingJustification(id.clone()));
    }
    if a.author.trim().is_empty() || a.approver.trim().is_empty() {
        return Err(BundleError::MissingAuthorOrApprover(id.clone()));
    }
    if a.author == a.approver {
        return Err(BundleError::ApproverIsAuthor(id.clone()));
    }
    if let Some(exp) = a.expires_at {
        if params.validation_time >= exp {
            return Err(BundleError::RecordExpired(id.clone()));
        }
    }
    Ok(())
}

/// Наибольшее число миров (§6.7): произведение по детекторам (число различных порогов + 1),
/// умноженное на (число различных констант `file.class` + 1).
fn world_bound(ir: &PolicyIr) -> u128 {
    let mut thresholds: BTreeMap<&DetectorId, BTreeSet<u32>> = BTreeMap::new();
    let mut classes = BTreeSet::new();
    for rule in &ir.rules {
        rule.condition.for_each_atom(&mut |a| match a {
            Atom::CountAtLeast(d, n) => {
                thresholds.entry(d).or_default().insert(*n);
            }
            Atom::ThresholdMet(d) => {
                let t = ir.detector_set.iter().find(|s| &s.detector_id == d).map_or(0, |s| s.threshold);
                thresholds.entry(d).or_default().insert(t);
            }
            Atom::FileClassIs(c) => {
                classes.insert(c);
            }
            _ => {}
        });
    }
    let det: u128 = thresholds.values().fold(1u128, |acc, s| acc.saturating_mul(s.len() as u128 + 1));
    det.saturating_mul(classes.len() as u128 + 1)
}
