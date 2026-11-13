//! Решение PDP (SENTINEL_CORE_DESIGN.md §5, §6.3, §6.6, §6.8; FAILURE_MODE_DESIGN.md §2.2).
//!
//! Отличия от эскиза §5, сделанные из-за пробелов контракта (перечислены в отчёте этапа 1):
//! * `policy_version`, `policy_bundle_id`, `verification_mode`, `capability_check` —
//!   `Option`: при `PolicyState` ABSENT, INVALID, EXPIRED (§6.3, шаг 1) проверенного бандла
//!   нет, и значения этих полей не определены.
//! * `ProvisionalDecision` — отдельный тип без преобразования в `Decision` (§2.3, §5): его
//!   нельзя передать туда, где ожидается финальное решение.

use crate::ids::{BundleId, DetectorId, FileClass, RecordId, RuleId};
use crate::inspection::{Classification, StructureEnumeration, TextCoverage};
use crate::reason::ReasonCode;

/// Версия схемы `Decision` (§11).
pub const SCHEMA_VERSION: u32 = 1;

/// Вердикт PDP.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Allow,
    Block,
    Indeterminate,
}

/// Действие правила. Порядок строгости: BLOCK > ALERT > MONITOR (§6.3, шаг 2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Monitor,
    Alert,
    Block,
}

impl Action {
    /// Ранг строгости: больше — строже.
    pub fn strictness(self) -> u8 {
        match self {
            Action::Monitor => 1,
            Action::Alert => 2,
            Action::Block => 3,
        }
    }
}

/// Fallback при INDETERMINATE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallback {
    AllowWithAudit,
    Block,
}

impl Fallback {
    /// Ранг строгости: больше — строже.
    pub fn strictness(self) -> u8 {
        match self {
            Fallback::AllowWithAudit => 0,
            Fallback::Block => 1,
        }
    }
}

/// Основание решения (§6.3, шаг 7).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecisionBasis {
    RuleMatched,
    NoRuleMatched,
    OutcomeIndependentOfUnknown,
    Indeterminate,
}

/// Одинаково ли действие во всех возможных исходах (только при ALLOW).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionCertainty {
    Definite,
    Possible,
}

/// Источник возможного действия.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionSourceRef {
    Rule(RuleId),
    IncompleteInspectionAction(RecordId),
}

/// Возможное действие и его источник.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionSource {
    pub source: ActionSourceRef,
    pub action: Action,
}

/// Режим загрузки бандла (§6.7).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerificationMode {
    EnforcementReady,
    InspectionOnly,
}

/// Выполнялась ли проверка возможностей платформы (§6.7).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapabilityCheck {
    Performed,
    Skipped,
}

/// Уровень раскрытия данных (D-07).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataLevel {
    Metadata,
    MaskedMatch,
}

/// Обработка входа действительности отступлений (§6.8).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviationInput {
    Provided,
    Unavailable,
    Contradictory,
}

/// Трёхзначное значение условия (логика Клини, §6.3, шаг 3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Truth {
    True,
    False,
    Unknown,
}

/// Основание выбора победителя перед следующим TRUE-правилом (§6.6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderBasis {
    Priority,
    Strictness,
    RuleId,
}

/// Итог одного применимого правила в объяснении.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleTrace {
    pub rule_id: RuleId,
    pub value: Truth,
    /// Атом, определивший итог (каноническая запись атома); `None`, если итог задан
    /// составным выражением без единственного решающего атома.
    pub determining_atom: Option<String>,
    /// Атом `inspection.incomplete` среди конъюнктов верхнего уровня.
    pub guards_incompleteness: bool,
}

/// Объяснение решения (§6.6). Значения совпадений, имена файлов и пути сюда не входят.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Explanation {
    pub core_version: String,
    pub detector_set_version: String,
    pub bundle_id: Option<BundleId>,
    pub policy_version: Option<u64>,
    /// Класс по сигнатуре; при неоднозначной классификации — кандидат.
    pub file_class: Option<FileClass>,
    pub classification: Option<Classification>,
    pub text_coverage: Option<TextCoverage>,
    pub structure_enumeration: Option<StructureEnumeration>,
    pub inspection_complete: bool,
    /// Детекторы: идентификатор, количество, порог достигнут (без значений).
    pub detectors: Vec<(DetectorId, u32, bool)>,
    /// Применимые правила в порядке ≻.
    pub rules: Vec<RuleTrace>,
    pub winner_basis: Option<OrderBasis>,
    /// Множество U (§6.3, шаг 4).
    pub unknown_rules: Vec<RuleId>,
    pub possible_winners: Vec<RuleId>,
    /// Есть ли мир без победителя.
    pub world_without_winner: bool,
    pub worlds_considered: u64,
    /// Значения класса, рассмотренные как измерение мира (`None` — OTHER).
    pub class_worlds: Vec<Option<FileClass>>,
    pub fallback_sources: Vec<RuleId>,
    pub part_exclusion_id: Option<RecordId>,
    pub precedence_approval_ids: Vec<RecordId>,
    pub suppressed_precedence_pairs: Vec<(RuleId, RuleId)>,
    pub incomplete_action_source: Option<RecordId>,
    pub capability_exception_id: Option<RecordId>,
    /// Правила с атомом неполноты, TRUE, но затенённые правилом выше по порядку.
    pub shadowed_incompleteness_guards: Vec<RuleId>,
    /// Противоречия входа `DeviationValidity` (§6.8).
    pub deviation_contradictions: Vec<String>,
    /// Причина INDETERMINATE по состоянию политики (§6.3, шаг 1).
    pub policy_state: Option<ReasonCode>,
}

/// Финальное решение PDP. Все поля и проекция данных берутся из одного вызова
/// `Pdp::decide` (§5, §6.8).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decision {
    pub verdict: Verdict,
    /// Непусто при INDETERMINATE (INV-C3).
    pub reasons: Vec<ReasonCode>,
    pub policy_version: Option<u64>,
    pub policy_bundle_id: Option<BundleId>,
    /// Правила со значением TRUE, включая проигравшие, в порядке ≻.
    pub matched_rules: Vec<RuleId>,
    /// Отсутствует только при `NO_RULE_MATCHED`.
    pub required_action: Option<Action>,
    pub decision_basis: DecisionBasis,
    pub inspection_complete: bool,
    pub winner_rule_id: Option<RuleId>,
    /// Заполнено, если возможных победителей несколько; порядок ≻.
    pub possible_winners: Vec<RuleId>,
    pub part_exclusion_id: Option<RecordId>,
    pub precedence_approval_ids: Vec<RecordId>,
    pub suppressed_precedence_pairs: Vec<(RuleId, RuleId)>,
    pub inactive_deviation_ids: Vec<RecordId>,
    pub deviation_input: DeviationInput,
    pub incomplete_action_source: Option<RecordId>,
    /// Только при ALLOW.
    pub action_certainty: Option<ActionCertainty>,
    /// Только при ALLOW; порядок ≻, запись контракта последней.
    pub action_sources: Vec<ActionSource>,
    pub verification_mode: Option<VerificationMode>,
    pub capability_check: Option<CapabilityCheck>,
    pub requested_action: Option<Action>,
    pub capability_exception_id: Option<RecordId>,
    /// Только при INDETERMINATE.
    pub fallback: Option<Fallback>,
    pub data_level: DataLevel,
    pub degraded: bool,
    pub explanation: Explanation,
    pub schema_version: u32,
}

/// Предварительное решение `Engine::evaluate` (§2.3, §5, §6.8). Отдельный тип: в
/// `Decision` не преобразуется, проекцией данных не принимается, PEP не передаётся.
/// `Engine::evaluate` на этапе 1 не реализован (нет извлечения); тип фиксирует контракт.
///
/// Диагностический доступ (CHG7-R2) ограничен [`ProvisionalSummary`]: ни ссылки, ни копии
/// `Decision` из предварительного решения получить нельзя, поэтому его нельзя передать туда,
/// где ожидается финальное решение. `Debug` печатает только сводку.
///
/// Предварительное решение не является `Decision` и не преобразуется в него:
///
/// ```compile_fail
/// use sentinel_core_types::{Decision, ProvisionalDecision};
/// fn final_only(_d: &Decision) {}
/// fn host(p: &ProvisionalDecision) {
///     final_only(p); // ошибка типов: ProvisionalDecision — не Decision
/// }
/// ```
///
/// ```compile_fail
/// use sentinel_core_types::{Decision, ProvisionalDecision};
/// fn host(p: ProvisionalDecision) -> Decision {
///     p.into() // From<ProvisionalDecision> for Decision не реализован
/// }
/// ```
///
/// Внутреннее поле недоступно, а диагностического доступа к `Decision` нет:
///
/// ```compile_fail
/// use sentinel_core_types::ProvisionalDecision;
/// fn host(p: &ProvisionalDecision) {
///     let _ = &p.inner; // поле приватное
/// }
/// ```
///
/// ```compile_fail
/// use sentinel_core_types::{Decision, ProvisionalDecision};
/// fn host(p: &ProvisionalDecision) -> Decision {
///     p.inspect().clone() // метода inspect нет (удалён по CHG7-R2)
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct ProvisionalDecision {
    inner: Decision,
}

/// Сводка предварительного решения для диагностики и журнала (CHG7-R2). Содержит только
/// скалярные поля и идентификаторы; из неё нельзя собрать `Decision` через API этого типа.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProvisionalSummary {
    pub verdict: Verdict,
    pub reasons: Vec<ReasonCode>,
    pub decision_basis: DecisionBasis,
    pub inspection_complete: bool,
    pub winner_rule_id: Option<RuleId>,
    pub possible_winners: Vec<RuleId>,
    pub deviation_input: DeviationInput,
}

impl ProvisionalDecision {
    /// Оборачивает решение, вычисленное по входу на момент формирования запроса.
    pub fn new(decision: Decision) -> Self {
        Self { inner: decision }
    }

    /// Диагностическая сводка; финального решения не содержит и в него не превращается.
    pub fn summary(&self) -> ProvisionalSummary {
        ProvisionalSummary {
            verdict: self.inner.verdict,
            reasons: self.inner.reasons.clone(),
            decision_basis: self.inner.decision_basis,
            inspection_complete: self.inner.inspection_complete,
            winner_rule_id: self.inner.winner_rule_id.clone(),
            possible_winners: self.inner.possible_winners.clone(),
            deviation_input: self.inner.deviation_input,
        }
    }
}

impl std::fmt::Debug for ProvisionalDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProvisionalDecision").field("summary", &self.summary()).finish_non_exhaustive()
    }
}
