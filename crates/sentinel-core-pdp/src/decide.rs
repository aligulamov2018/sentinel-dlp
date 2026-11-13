//! Алгоритм оценки PDP (SENTINEL_CORE_DESIGN.md §6.3, §6.6, §6.8; Design Accepted CHG-0005).

use crate::eval::{worlds, Inputs};
use crate::policy::{conflict_pairs, ExcludableKind, PartInspectionExclusion, PolicyIr, Rule};
use crate::{CompiledPolicy, EngineFault, InvalidRequest, Mode, PolicyState, PolicyStateKind};
use sentinel_core_types::{
    Action, ActionCertainty, ActionSource, ActionSourceRef, Classification, DataLevel, Decision,
    DecisionBasis, DecisionContext, DeviationInput, DeviationValidity, Explanation, Fallback,
    InspectionResult, OrderBasis, PartKind, PolicyValidity, ReasonCode, RecordId, RuleId, RuleTrace,
    StructureEnumeration, TextCoverage, TimeState, Truth, Verdict, SCHEMA_VERSION,
};
use std::collections::BTreeSet;

/// Версия ядра в объяснении.
const CORE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Результат нормализации входа действительности отступлений (§6.8).
struct Validity {
    valid: BTreeSet<RecordId>,
    input: DeviationInput,
    contradictions: Vec<String>,
}

/// Нормализация `DeviationValidity` (§6.8, таблица «Отсутствующий или противоречивый вход»).
/// Отсутствующий или противоречивый вход: все отступления недействительны; `EngineFault` нет.
fn normalize_validity(dv: &DeviationValidity, deviation_ids: &BTreeSet<RecordId>) -> Validity {
    match dv {
        DeviationValidity::Unavailable => Validity {
            valid: BTreeSet::new(),
            input: DeviationInput::Unavailable,
            contradictions: Vec::new(),
        },
        DeviationValidity::Provided { time_state, valid_ids } => {
            let mut contradictions = Vec::new();
            if *time_state == TimeState::TimeUnverifiable && !valid_ids.is_empty() {
                contradictions.push("TIME_UNVERIFIABLE при непустом valid_ids".to_string());
            }
            let unknown: BTreeSet<&RecordId> =
                valid_ids.iter().filter(|id| !deviation_ids.contains(*id)).collect();
            for id in unknown {
                contradictions
                    .push(format!("идентификатор {id} не является записью-отступлением загруженного бандла"));
            }
            if contradictions.is_empty() {
                // Повторяющиеся идентификаторы нормализуются в множество.
                Validity {
                    valid: valid_ids.iter().cloned().collect(),
                    input: DeviationInput::Provided,
                    contradictions,
                }
            } else {
                Validity { valid: BTreeSet::new(), input: DeviationInput::Contradictory, contradictions }
            }
        }
    }
}

fn degraded(ctx: &DecisionContext) -> bool {
    // Толкование этапа 1: семантика поля `degraded` в §5 не определена; решение помечается,
    // если политика оценивается не в состоянии времени CURRENT (FAILURE_MODE_DESIGN.md §4.6).
    ctx.policy_validity != PolicyValidity::Current
}

fn sorted_reasons(rs: &[ReasonCode]) -> Vec<ReasonCode> {
    let set: BTreeSet<ReasonCode> = rs.iter().copied().collect();
    set.into_iter().collect()
}

fn required_for_fallback(fb: Fallback) -> Action {
    match fb {
        Fallback::Block => Action::Block,
        Fallback::AllowWithAudit => Action::Monitor,
    }
}

/// Применимо ли исключение анализа частей к объекту (§4.2, таблица условий).
fn exclusion_applies(ex: &PartInspectionExclusion, insp: &InspectionResult, ctx: &DecisionContext) -> bool {
    if !ex.scope.matches(ctx) {
        return false;
    }
    if insp.classification != Classification::Confirmed
        || insp.structure_enumeration != StructureEnumeration::Complete
        || insp.text_coverage != TextCoverage::Complete
        || insp.uninspected_parts.is_empty()
    {
        return false;
    }
    insp.uninspected_parts.iter().all(|p| {
        let kind = match p.kind {
            PartKind::Image => ExcludableKind::Image,
            PartKind::EmbeddedObject => ExcludableKind::EmbeddedObject,
            PartKind::Attachment => ExcludableKind::Attachment,
            PartKind::Unknown => return false,
        };
        ex.kinds.contains(&kind)
    })
}

/// Решение без проверенного бандла: ABSENT, INVALID, EXPIRED (§6.3, шаг 1).
pub(crate) fn decide_without_policy(
    state: &PolicyState,
    _ctx: &DecisionContext, // симметрия с decide; без бандла контекст на решение не влияет
    insp: &InspectionResult,
    dv: &DeviationValidity,
) -> Result<Decision, EngineFault> {
    insp.check_contract().map_err(|v| EngineFault::InvalidRequest(InvalidRequest::InspectionContract(v)))?;
    let reason = match state.kind {
        PolicyStateKind::Absent => ReasonCode::PolicyAbsent,
        PolicyStateKind::Invalid => ReasonCode::PolicyInvalid,
        PolicyStateKind::Expired => ReasonCode::PolicyExpired,
    };
    // Бандла нет: записей-отступлений нет, любой переданный идентификатор противоречив.
    let validity = normalize_validity(dv, &BTreeSet::new());
    let fb = state.bootstrap_fallback;
    Ok(Decision {
        verdict: Verdict::Indeterminate,
        reasons: vec![reason],
        policy_version: None,
        policy_bundle_id: None,
        matched_rules: Vec::new(),
        required_action: Some(required_for_fallback(fb)),
        decision_basis: DecisionBasis::Indeterminate,
        inspection_complete: insp.inspection_complete(),
        winner_rule_id: None,
        possible_winners: Vec::new(),
        part_exclusion_id: None,
        precedence_approval_ids: Vec::new(),
        suppressed_precedence_pairs: Vec::new(),
        inactive_deviation_ids: Vec::new(),
        deviation_input: validity.input,
        incomplete_action_source: None,
        action_certainty: None,
        action_sources: Vec::new(),
        verification_mode: None,
        capability_check: None,
        requested_action: None,
        capability_exception_id: None,
        fallback: Some(fb),
        data_level: DataLevel::Metadata,
        // Толкование этапа 1: решение без проверенного бандла всегда помечается как деградированное.
        degraded: true,
        explanation: Explanation {
            core_version: CORE_VERSION.to_string(),
            detector_set_version: insp.engine.detector_set_version.clone(),
            file_class: Some(insp.file_class.clone()),
            classification: Some(insp.classification),
            text_coverage: Some(insp.text_coverage),
            structure_enumeration: Some(insp.structure_enumeration),
            inspection_complete: insp.inspection_complete(),
            deviation_contradictions: validity.contradictions,
            policy_state: Some(reason),
            ..Explanation::default()
        },
        schema_version: SCHEMA_VERSION,
    })
}

/// Исход возможного победителя: правило или «победителя нет» (`None`).
type Winner = Option<usize>;

/// Решение по проверенному бандлу (§6.3, шаги 2–9).
pub(crate) fn decide<M: Mode>(
    policy: &CompiledPolicy<M>,
    ctx: &DecisionContext,
    insp: &InspectionResult,
    dv: &DeviationValidity,
) -> Result<Decision, EngineFault> {
    insp.check_contract().map_err(|v| EngineFault::InvalidRequest(InvalidRequest::InspectionContract(v)))?;
    if ctx.policy_validity == PolicyValidity::Expired {
        // §6.3, шаг 1: fallback при EXPIRED берётся из bootstrap-политики, которую хост
        // передаёт как PolicyState. Проверенный бандл с EXPIRED — противоречивый вызов.
        return Err(EngineFault::InvalidRequest(InvalidRequest::ExpiredPolicyPassedAsCompiled));
    }
    let ir: &PolicyIr = policy.validated().ir();

    // §6.8: нормализация входа действительности.
    let deviation_ids: BTreeSet<RecordId> = ir
        .exclusions
        .iter()
        .map(|e| e.exclusion_id.clone())
        .chain(ir.approvals.iter().map(|a| a.approval_id.clone()))
        .collect();
    let validity = normalize_validity(dv, &deviation_ids);

    // Шаг 2: применимость и порядок (порядок проверен при загрузке).
    let app: Vec<&Rule> = ir.rules.iter().filter(|r| r.scope.matches(ctx)).collect();

    // §4.2: исключение анализа частей — запись с наименьшим идентификатором среди применимых.
    let mut exclusions: Vec<&PartInspectionExclusion> = ir
        .exclusions
        .iter()
        .filter(|e| validity.valid.contains(&e.exclusion_id) && exclusion_applies(e, insp, ctx))
        .collect();
    exclusions.sort_by(|a, b| a.exclusion_id.cmp(&b.exclusion_id));
    let applied_exclusion = exclusions.first().map(|e| e.exclusion_id.clone());

    let complete = insp.inspection_complete();
    let inputs = Inputs {
        insp,
        ctx,
        detectors: &ir.detector_set,
        two_valued: complete || applied_exclusion.is_some(),
        complete,
    };

    // Шаг 2: превосходство в конфликтной паре (§6.7, §6.8).
    let pairs = conflict_pairs(&app);
    let mut suppressed: Vec<(usize, usize)> = Vec::new();
    let mut approvals_used: BTreeSet<RecordId> = BTreeSet::new();
    for p in &pairs {
        let (h, l) = (app[p.higher], app[p.lower]);
        let approval = ir
            .approvals
            .iter()
            .filter(|a| {
                a.higher_rule_id == h.rule_id
                    && a.lower_rule_id == l.rule_id
                    && validity.valid.contains(&a.approval_id)
                    && a.scope.matches(ctx)
            })
            .map(|a| a.approval_id.clone())
            .min();
        match approval {
            Some(id) => {
                approvals_used.insert(id);
            }
            None => suppressed.push((p.higher, p.lower)),
        }
    }
    let is_suppressed = |h: usize, l: usize| suppressed.contains(&(h, l));

    // Шаг 3: значения условий.
    let traces: Vec<(Truth, Option<String>)> = app.iter().map(|r| inputs.expr(&r.condition)).collect();

    // Шаг 4: W и U.
    let w = traces.iter().position(|(v, _)| *v == Truth::True);
    let above = w.unwrap_or(app.len());
    let unknown: Vec<usize> = (0..above).filter(|i| traces[*i].0 == Truth::Unknown).collect();

    // Шаги 5–6: возможные победители.
    let winner_in = |truth: &dyn Fn(usize) -> bool, sup: bool| -> Winner {
        (0..app.len())
            .find(|&r| truth(r) && !(sup && (0..app.len()).any(|l| truth(l) && is_suppressed(r, l))))
    };
    let mut winners: Vec<Winner> = Vec::new();
    let mut unsuppressed_winners: Vec<usize> = Vec::new();
    let mut worlds_considered: u64 = 1;
    let mut class_worlds = Vec::new();
    if unknown.is_empty() && suppressed.is_empty() {
        winners.push(w);
    } else {
        let (ws, cw) = worlds(&app, &inputs);
        worlds_considered = ws.len() as u64;
        class_worlds = cw;
        for world in &ws {
            let vals: Vec<bool> = app.iter().map(|r| inputs.in_world(&r.condition, world)).collect();
            let truth = |i: usize| vals[i];
            let win = winner_in(&truth, true);
            if !winners.contains(&win) {
                winners.push(win);
            }
            if !suppressed.is_empty() {
                if let Some(u) = winner_in(&truth, false) {
                    if !unsuppressed_winners.contains(&u) {
                        unsuppressed_winners.push(u);
                    }
                }
            }
        }
    }
    let mut rule_winners: Vec<usize> = winners.iter().flatten().copied().collect();
    rule_winners.sort_unstable();
    let world_without_winner = winners.contains(&None);

    // Шаг 7: исходы. Действие исхода «победителя нет» при неполном покрытии — из контракта.
    let contract = if !complete && world_without_winner {
        let mut recs: Vec<_> = ir.incomplete_actions.iter().filter(|r| r.scope.matches(ctx)).collect();
        recs.sort_by(|a, b| {
            b.action
                .as_action()
                .strictness()
                .cmp(&a.action.as_action().strictness())
                .then(a.record_id.cmp(&b.record_id))
        });
        // Запись с неограниченной областью обязательна (§6.7), поэтому применимая есть всегда.
        match recs.first() {
            Some(r) => Some((r.record_id.clone(), r.action.as_action())),
            None => return Err(EngineFault::Internal("нет применимой записи IncompleteInspectionAction")),
        }
    } else {
        None
    };
    let any_block = rule_winners.iter().any(|&i| app[i].action == Action::Block);
    let any_allow = world_without_winner || rule_winners.iter().any(|&i| app[i].action != Action::Block);

    let verdict;
    let mut required_action = None;
    let decision_basis;
    let mut action_certainty = None;
    let mut action_sources = Vec::new();
    let mut fallback = None;
    let mut reasons = Vec::new();
    let mut fallback_sources = Vec::new();
    let mut incomplete_action_source = None;

    if any_block && !any_allow {
        verdict = Verdict::Block;
        required_action = Some(Action::Block);
        decision_basis = DecisionBasis::RuleMatched;
    } else if any_allow && !any_block {
        verdict = Verdict::Allow;
        if winners == [None] && complete {
            decision_basis = DecisionBasis::NoRuleMatched;
        } else {
            decision_basis = if world_without_winner {
                DecisionBasis::OutcomeIndependentOfUnknown
            } else {
                DecisionBasis::RuleMatched
            };
            // Возможные действия и их источники: порядок ≻, запись контракта последней.
            let mut actions: Vec<(ActionSourceRef, Action)> = rule_winners
                .iter()
                .map(|&i| (ActionSourceRef::Rule(app[i].rule_id.clone()), app[i].action))
                .collect();
            if let Some((id, a)) = &contract {
                actions.push((ActionSourceRef::IncompleteInspectionAction(id.clone()), *a));
                incomplete_action_source = Some(id.clone());
            }
            let strictest = actions.iter().map(|(_, a)| *a).max_by_key(|a| a.strictness());
            required_action = strictest;
            let distinct: BTreeSet<u8> = actions.iter().map(|(_, a)| a.strictness()).collect();
            action_certainty =
                Some(if distinct.len() == 1 { ActionCertainty::Definite } else { ActionCertainty::Possible });
            let mut seen = BTreeSet::new();
            for (src, a) in actions {
                if seen.insert(a.strictness()) {
                    action_sources.push(ActionSource { source: src, action: a });
                }
            }
        }
    } else {
        // Исходы различаются по вердикту → INDETERMINATE (шаг 8).
        verdict = Verdict::Indeterminate;
        decision_basis = DecisionBasis::Indeterminate;
        reasons = sorted_reasons(&insp.reasons);
        if reasons.is_empty() {
            // При полном покрытии и одном мире различие исходов невозможно (INV-C3).
            return Err(EngineFault::Internal("INDETERMINATE без причин неполноты"));
        }
        let mut contributors: Vec<usize> = rule_winners.clone();
        for u in &unsuppressed_winners {
            if !contributors.contains(u) {
                contributors.push(*u);
            }
        }
        contributors.sort_unstable();
        let fb = contributors
            .iter()
            .map(|&i| app[i].on_indeterminate)
            .max_by_key(|f| f.strictness())
            .unwrap_or(Fallback::AllowWithAudit);
        fallback = Some(fb);
        required_action = Some(required_for_fallback(fb));
        fallback_sources = contributors.iter().map(|&i| app[i].rule_id.clone()).collect();
    }

    // Шаг 9: победитель и уровень данных.
    let single = winners.len() == 1;
    let winner_rule = if single { winners[0] } else { None };
    let winner_rule_id = winner_rule.map(|i| app[i].rule_id.clone());
    let possible_winners: Vec<RuleId> = if single {
        Vec::new()
    } else {
        let mut ids: Vec<RuleId> = rule_winners.iter().map(|&i| app[i].rule_id.clone()).collect();
        ids.sort();
        ids
    };
    let data_level = if verdict == Verdict::Indeterminate || world_without_winner {
        DataLevel::Metadata
    } else if single {
        winner_rule.map_or(DataLevel::Metadata, |i| app[i].data_level)
    } else {
        rule_winners.iter().map(|&i| app[i].data_level).min().unwrap_or(DataLevel::Metadata)
    };
    let (requested_action, capability_exception_id) = match winner_rule {
        Some(i) => (app[i].requested_action, app[i].capability_exception_id.clone()),
        None => (None, None),
    };

    let matched_rules: Vec<RuleId> =
        (0..app.len()).filter(|&i| traces[i].0 == Truth::True).map(|i| app[i].rule_id.clone()).collect();
    let suppressed_pairs: Vec<(RuleId, RuleId)> =
        suppressed.iter().map(|&(h, l)| (app[h].rule_id.clone(), app[l].rule_id.clone())).collect();

    // Записи-отступления, недействительные в момент решения (§5, §6.8) — все записи бандла вне
    // множества действительных, без фильтра по области (буквальное чтение §5; ИТ-07 снято).
    let mut inactive: Vec<RecordId> =
        deviation_ids.iter().filter(|id| !validity.valid.contains(*id)).cloned().collect();
    inactive.sort();

    let winner_basis = w.and_then(|wi| {
        if winner_rule != Some(wi) {
            return None;
        }
        let next = (wi + 1..app.len()).find(|&i| traces[i].0 == Truth::True)?;
        let (a, b) = (app[wi], app[next]);
        Some(if a.priority != b.priority {
            OrderBasis::Priority
        } else if a.action != b.action {
            OrderBasis::Strictness
        } else {
            OrderBasis::RuleId
        })
    });
    let shadowed_guards: Vec<RuleId> = match winner_rule {
        Some(wi) => (wi + 1..app.len())
            .filter(|&i| traces[i].0 == Truth::True && app[i].condition.guards_incompleteness())
            .map(|i| app[i].rule_id.clone())
            .collect(),
        None => Vec::new(),
    };

    let explanation = Explanation {
        core_version: CORE_VERSION.to_string(),
        detector_set_version: insp.engine.detector_set_version.clone(),
        bundle_id: Some(ir.bundle_id.clone()),
        policy_version: Some(ir.policy_version),
        file_class: Some(insp.file_class.clone()),
        classification: Some(insp.classification),
        text_coverage: Some(insp.text_coverage),
        structure_enumeration: Some(insp.structure_enumeration),
        inspection_complete: complete,
        detectors: insp
            .detections
            .iter()
            .map(|d| (d.detector_id.clone(), d.match_count, d.threshold_met))
            .collect(),
        rules: app
            .iter()
            .zip(&traces)
            .map(|(r, (v, at))| RuleTrace {
                rule_id: r.rule_id.clone(),
                value: *v,
                determining_atom: at.clone(),
                guards_incompleteness: r.condition.guards_incompleteness(),
            })
            .collect(),
        winner_basis,
        unknown_rules: unknown.iter().map(|&i| app[i].rule_id.clone()).collect(),
        possible_winners: rule_winners.iter().map(|&i| app[i].rule_id.clone()).collect(),
        world_without_winner,
        worlds_considered,
        class_worlds,
        fallback_sources,
        part_exclusion_id: applied_exclusion.clone(),
        precedence_approval_ids: approvals_used.iter().cloned().collect(),
        suppressed_precedence_pairs: suppressed_pairs.clone(),
        incomplete_action_source: incomplete_action_source.clone(),
        capability_exception_id: capability_exception_id.clone(),
        shadowed_incompleteness_guards: shadowed_guards,
        deviation_contradictions: validity.contradictions,
        policy_state: None,
    };

    Ok(Decision {
        verdict,
        reasons,
        policy_version: Some(ir.policy_version),
        policy_bundle_id: Some(ir.bundle_id.clone()),
        matched_rules,
        required_action,
        decision_basis,
        inspection_complete: complete,
        winner_rule_id,
        possible_winners,
        part_exclusion_id: applied_exclusion,
        precedence_approval_ids: approvals_used.into_iter().collect(),
        suppressed_precedence_pairs: suppressed_pairs,
        inactive_deviation_ids: inactive,
        deviation_input: validity.input,
        incomplete_action_source,
        action_certainty,
        action_sources,
        verification_mode: Some(M::VERIFICATION_MODE),
        capability_check: Some(M::CAPABILITY_CHECK),
        requested_action,
        capability_exception_id,
        fallback,
        data_level,
        degraded: degraded(ctx),
        explanation,
        schema_version: SCHEMA_VERSION,
    })
}
