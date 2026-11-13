//! PDP `sentinel-core` (Design Accepted CHG-0005; docs/architecture/SENTINEL_CORE_DESIGN.md §6).
//!
//! Этап 1 (решение владельца 2026-11-12): модель политики, структурная проверка бандла (§6.7,
//! строки без подписи и манифеста), алгоритм оценки (§6.3), обработка `DeviationValidity`
//! (§6.8). Только стандартная библиотека.
//!
//! Не реализовано на этапе 1 и не заявляется:
//! * проверка подписи бандла и загрузчики `PolicyBundle::load_enforcement_ready` /
//!   `load_inspection_only` (вопрос O-05), формат бандла (O-06);
//! * проверка манифеста возможностей и строки §6.7 режима `EnforcementReady`;
//! * извлечение, детекторы, `Engine::evaluate`, проекция данных, CLI, изоляция разбора.
//!
//! Поэтому `CompiledPolicy` не имеет публичного конструктора: получить её можно только через
//! загрузчик с проверкой подписи, которого пока нет. Тесты используют конструктор,
//! доступный только в `cfg(test)`.

#![forbid(unsafe_code)]

mod decide;
mod eval;
pub mod policy;
pub mod validate;

#[cfg(test)]
mod tests;

use sentinel_core_types::{
    CapabilityCheck, Decision, DecisionContext, DeviationValidity, Fallback, InspectionContractViolation,
    InspectionResult, VerificationMode,
};
use std::marker::PhantomData;

pub use validate::{BundleError, ValidatedPolicy, ValidationParams, DEFAULT_WORLD_LIMIT};

mod sealed {
    pub trait Sealed {}
}

/// Режим загрузки бандла (§6.7). Различается типом результата, а не флагом.
pub trait Mode: sealed::Sealed {
    const VERIFICATION_MODE: VerificationMode;
    const CAPABILITY_CHECK: CapabilityCheck;
}

/// Режим PEP: все проверки §6.7, включая манифест возможностей.
#[derive(Debug, Clone, Copy)]
pub enum EnforcementReady {}

/// Режим аналитики без PEP: подпись и состав проверяются, возможности платформы — нет.
#[derive(Debug, Clone, Copy)]
pub enum InspectionOnly {}

impl sealed::Sealed for EnforcementReady {}
impl sealed::Sealed for InspectionOnly {}

impl Mode for EnforcementReady {
    const VERIFICATION_MODE: VerificationMode = VerificationMode::EnforcementReady;
    const CAPABILITY_CHECK: CapabilityCheck = CapabilityCheck::Performed;
}

impl Mode for InspectionOnly {
    const VERIFICATION_MODE: VerificationMode = VerificationMode::InspectionOnly;
    const CAPABILITY_CHECK: CapabilityCheck = CapabilityCheck::Skipped;
}

/// Проверенная политика заданного режима (§2.3, §6.7).
#[derive(Debug, Clone)]
pub struct CompiledPolicy<M: Mode> {
    policy: ValidatedPolicy,
    _mode: PhantomData<M>,
}

impl<M: Mode> CompiledPolicy<M> {
    /// Проверенное IR.
    pub fn validated(&self) -> &ValidatedPolicy {
        &self.policy
    }
}

impl CompiledPolicy<InspectionOnly> {
    /// Только для тестов этапа 1: политика без проверки подписи. Загрузчика с подписью нет
    /// (O-05), поэтому вне `cfg(test)` этот конструктор недоступен.
    #[cfg(test)]
    pub(crate) fn unsigned_for_tests(policy: ValidatedPolicy) -> Self {
        Self { policy, _mode: PhantomData }
    }
}

/// Вид состояния политики без проверенного бандла (§6.3, шаг 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyStateKind {
    /// Проверенной политики не было никогда.
    Absent,
    /// Сохранённый действующий бандл не прошёл повторную проверку, резервной копии нет.
    Invalid,
    /// `policy_validity = EXPIRED`.
    Expired,
}

/// Состояние политики, которое передаёт хост вместо проверенного бандла; `bootstrap_fallback`
/// — fallback bootstrap-политики хоста (FAILURE_MODE_DESIGN.md §6.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyState {
    pub kind: PolicyStateKind,
    pub bootstrap_fallback: Fallback,
}

/// Ошибка вызова или внутренняя ошибка PDP (§2.3). Хост трактует любую как отсутствие вердикта.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineFault {
    InvalidRequest(InvalidRequest),
    Internal(&'static str),
}

/// Некорректный запрос.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidRequest {
    /// Результат инспекции нарушает правила статусов §5.
    InspectionContract(InspectionContractViolation),
    /// `policy_validity = EXPIRED` при переданном проверенном бандле: хост должен передать
    /// `PolicyState { kind: Expired, .. }` с fallback bootstrap-политики (§6.3, шаг 1).
    ExpiredPolicyPassedAsCompiled,
}

/// Точка принятия решения.
#[derive(Debug, Clone, Copy)]
pub struct Pdp;

impl Pdp {
    /// Финальное решение по проверенному бандлу; `DeviationValidity` — на момент этого вызова
    /// (§2.3, §6.8, INV-H3).
    pub fn decide<M: Mode>(
        policy: &CompiledPolicy<M>,
        ctx: &DecisionContext,
        insp: &InspectionResult,
        deviations: &DeviationValidity,
    ) -> Result<Decision, EngineFault> {
        decide::decide(policy, ctx, insp, deviations)
    }

    /// Решение без проверенного бандла: INDETERMINATE(POLICY_ABSENT | POLICY_INVALID |
    /// POLICY_EXPIRED) с fallback bootstrap-политики (§6.3, шаг 1).
    pub fn decide_without_policy(
        state: &PolicyState,
        ctx: &DecisionContext,
        insp: &InspectionResult,
        deviations: &DeviationValidity,
    ) -> Result<Decision, EngineFault> {
        decide::decide_without_policy(state, ctx, insp, deviations)
    }
}
