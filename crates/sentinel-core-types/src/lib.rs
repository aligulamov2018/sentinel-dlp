//! Контракты `sentinel-core` (Design Accepted CHG-0005, docs/architecture/SENTINEL_CORE_DESIGN.md).
//!
//! Crate содержит только типы данных и их производные свойства. Логики инспекции, загрузки
//! бандла и принятия решения здесь нет: PDP — `sentinel-core-pdp`, извлечение и детекторы —
//! будущие crates (парсеры и проверка подписи вне этого этапа: вопросы O-04, O-05).
//!
//! Каноническое представление `Decision` и `InspectionResult` для проверки детерминизма (§6.5,
//! TC-CORE-15; CHG-0008) — модуль `canonical` при cargo-функции `canonical`, выключенной по
//! умолчанию. Это не формат CLI, IPC, бандла или событий; разбора нет; внешние crates не
//! используются.

#![forbid(unsafe_code)]

#[cfg(feature = "canonical")]
pub mod canonical;
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
