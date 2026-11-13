//! Контракты `sentinel-core` (Design Accepted CHG-0005, docs/architecture/SENTINEL_CORE_DESIGN.md).
//!
//! Crate содержит только типы данных и их производные свойства. Логики инспекции, загрузки
//! бандла и принятия решения здесь нет: PDP — `sentinel-core-pdp`, извлечение и детекторы —
//! будущие crates (парсеры и проверка подписи вне этого этапа: вопросы O-04, O-05).
//!
//! Сериализация (§6.5, «каноническая сериализация `Decision`») на этом этапе не реализована:
//! формат — вопрос O-06; внешние crates не используются.

#![forbid(unsafe_code)]

pub mod context;
pub mod decision;
pub mod deviation;
pub mod ids;
pub mod inspection;
pub mod reason;

pub use context::{DecisionContext, PolicyValidity, ScopeAttr};
pub use decision::{
    Action, ActionCertainty, ActionSource, ActionSourceRef, CapabilityCheck, DataLevel, Decision,
    DecisionBasis, DeviationInput, Explanation, Fallback, OrderBasis, ProvisionalDecision,
    ProvisionalSummary, RuleTrace, Truth, Verdict, VerificationMode, SCHEMA_VERSION,
};
pub use deviation::{DeviationValidity, TimeState};
pub use ids::{
    BundleId, Channel, DetectorId, DeviceRef, FileClass, GroupId, Operation, OrgUnitId, PrincipalRef,
    RecordId, RuleId,
};
pub use inspection::{
    AmbiguityReason, Classification, ContentDigest, DetectionResult, DigestStatus, DigestTrust, EngineInfo,
    InspectionContractViolation, InspectionOrigin, InspectionResult, InspectionStatus, ObjectCoverage,
    PartKind, StructureEnumeration, TextCoverage, UninspectedPart,
};
pub use reason::ReasonCode;
