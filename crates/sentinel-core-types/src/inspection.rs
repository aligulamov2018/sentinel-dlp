//! Результат инспекции (SENTINEL_CORE_DESIGN.md §4.1–§4.3, §5).
//!
//! Отличия от эскиза §5, сделанные из-за пробелов контракта (перечислены в отчёте этапа 1):
//! * `file_class` — класс файла по сигнатуре. В эскизе `InspectionResult` его нет, но атом
//!   `file.class` (§6.1) и объяснение решения (§6.6) требуют класс на входе PDP. При
//!   `classification = AMBIGUOUS` это только кандидат (§4.3, INV-C14).
//! * `object_coverage` и `inspection_complete` — производные величины (§5: «производное»),
//!   поэтому они вычисляются методами и не могут разойтись с исходными полями.
//! * Внутренние значения совпадений (`findings`) не входят в этот тип: они не покидают процесс
//!   инспекции (INV-C6) и относятся к будущему crate детекторов.

use crate::ids::{DetectorId, FileClass};
use crate::reason::ReasonCode;

/// Статус инспекции.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InspectionStatus {
    Match,
    NoMatch,
    NotInspected,
}

/// Полнота разбора текстовых частей.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextCoverage {
    Complete,
    Partial,
}

/// Достоверность перечисления частей объекта.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructureEnumeration {
    Complete,
    Incomplete(ReasonCode),
}

/// Производная величина §4.2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectCoverage {
    Full,
    Incomplete,
}

/// Причина неоднозначной классификации (§4.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AmbiguityReason {
    DeclaredContainerMismatch,
    EmbeddedSignature,
}

/// Достоверность определения класса.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Classification {
    Confirmed,
    Ambiguous(AmbiguityReason),
}

/// Вид доказанной неразобранной части (§4.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartKind {
    Image,
    EmbeddedObject,
    Attachment,
    Unknown,
}

/// Доказанная неразобранная часть.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UninspectedPart {
    pub kind: PartKind,
    pub count: u32,
}

/// Результат одного детектора (без значений совпадений).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetectionResult {
    pub detector_id: DetectorId,
    pub detector_version: String,
    pub match_count: u32,
    pub score: u32,
    pub threshold_met: bool,
}

/// Происхождение результата инспекции (§10.2, ADR-0018 §5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InspectionOrigin {
    Core,
    HostSynthesized,
}

/// Статус полного хеша (§3.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DigestStatus {
    Computed,
    PartialRead,
    SizeLimit,
    ReadError,
    NotRequested,
    NoSnapshot,
}

/// Доверие к хешу (§3.2, INV-C8).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DigestTrust {
    TrustedSnapshot,
    Untrusted,
}

/// Хеши содержимого (§3.2). Вычисление — вне этапа 1; здесь только контракт данных.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentDigest {
    pub full: Option<[u8; 32]>,
    pub full_status: DigestStatus,
    pub read_set_digest: [u8; 32],
    pub trust: DigestTrust,
}

impl ContentDigest {
    /// Хеш не вычислялся: недоверенный результат без полного хеша.
    pub fn not_computed(status: DigestStatus) -> Self {
        Self { full: None, full_status: status, read_set_digest: [0; 32], trust: DigestTrust::Untrusted }
    }
}

/// Версии компонентов, давших результат.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineInfo {
    pub core_version: String,
    pub detector_set_version: String,
    pub limits_profile_id: String,
}

/// Результат инспекции — вход PDP.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InspectionResult {
    pub status: InspectionStatus,
    /// Непусто при `NOT_INSPECTED` и при любой неполноте покрытия, в том числе при `MATCH`.
    pub reasons: Vec<ReasonCode>,
    pub text_coverage: TextCoverage,
    pub structure_enumeration: StructureEnumeration,
    pub classification: Classification,
    pub uninspected_parts: Vec<UninspectedPart>,
    /// Отсортированы по `detector_id`, без повторов.
    pub detections: Vec<DetectionResult>,
    /// Класс по сигнатуре; при `AMBIGUOUS` — только кандидат (см. документацию модуля).
    pub file_class: FileClass,
    pub digest: ContentDigest,
    pub engine: EngineInfo,
    pub inspection_origin: InspectionOrigin,
}

/// Нарушение правил статусов §5 или порядка полей.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InspectionContractViolation {
    NoMatchWithoutFullCoverage,
    MatchWithoutThreshold,
    NotInspectedWithThreshold,
    NoMatchWithThreshold,
    MissingReasons,
    DetectionsNotSorted,
    PolicyReasonInInspection(ReasonCode),
}

impl InspectionResult {
    /// `FULL` — только при полном перечислении и пустом `uninspected_parts` (§4.2).
    pub fn object_coverage(&self) -> ObjectCoverage {
        if self.structure_enumeration == StructureEnumeration::Complete && self.uninspected_parts.is_empty() {
            ObjectCoverage::Full
        } else {
            ObjectCoverage::Incomplete
        }
    }

    /// Полное покрытие (§4.2): текст полный, объект полный, класс подтверждён.
    pub fn inspection_complete(&self) -> bool {
        self.text_coverage == TextCoverage::Complete
            && self.object_coverage() == ObjectCoverage::Full
            && self.classification == Classification::Confirmed
    }

    /// Результат детектора, если он есть в списке.
    pub fn detection(&self, id: &DetectorId) -> Option<&DetectionResult> {
        self.detections.binary_search_by(|d| d.detector_id.cmp(id)).ok().map(|i| &self.detections[i])
    }

    /// Найденное количество совпадений детектора (0, если детектора нет в списке).
    pub fn found_count(&self, id: &DetectorId) -> u32 {
        self.detection(id).map_or(0, |d| d.match_count)
    }

    /// Проверка правил статусов §5 (INV-C4) и формы полей. PDP не принимает результат,
    /// нарушающий контракт: это ошибка вызова, а не неполнота инспекции.
    pub fn check_contract(&self) -> Result<(), InspectionContractViolation> {
        if self.detections.windows(2).any(|w| w[0].detector_id >= w[1].detector_id) {
            return Err(InspectionContractViolation::DetectionsNotSorted);
        }
        if let Some(r) = self.reasons.iter().find(|r| !r.is_inspection_reason()) {
            return Err(InspectionContractViolation::PolicyReasonInInspection(*r));
        }
        let complete = self.inspection_complete();
        let any_met = self.detections.iter().any(|d| d.threshold_met);
        match self.status {
            InspectionStatus::NoMatch if !complete => {
                return Err(InspectionContractViolation::NoMatchWithoutFullCoverage)
            }
            InspectionStatus::NoMatch if any_met => {
                return Err(InspectionContractViolation::NoMatchWithThreshold)
            }
            InspectionStatus::Match if !any_met => {
                return Err(InspectionContractViolation::MatchWithoutThreshold)
            }
            InspectionStatus::NotInspected if any_met => {
                return Err(InspectionContractViolation::NotInspectedWithThreshold)
            }
            _ => {}
        }
        let needs_reasons = !complete || self.status == InspectionStatus::NotInspected;
        if needs_reasons && self.reasons.is_empty() {
            return Err(InspectionContractViolation::MissingReasons);
        }
        Ok(())
    }
}
