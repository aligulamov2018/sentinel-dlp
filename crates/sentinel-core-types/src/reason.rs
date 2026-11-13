//! Коды причин (FAILURE_MODE_DESIGN.md §2.3; SENTINEL_CORE_DESIGN.md §5).
//!
//! `SERVICE_UNAVAILABLE`, `QUEUE_SATURATED`, `AGENT_NOT_READY` формирует хост, не ядро,
//! поэтому в этом перечне их нет. Неизвестный код от другой версии трактуется как
//! `ENGINE_ERROR` (§10.2) — это правило десериализации; сериализация на этапе 1 не реализована.

/// Код причины неполноты инспекции или INDETERMINATE.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReasonCode {
    Timeout,
    UnsupportedFormat,
    ContentEncrypted,
    ResourceExhausted,
    ParseError,
    EngineError,
    PolicyAbsent,
    PolicyInvalid,
    PolicyExpired,
    ContentChanged,
}

impl ReasonCode {
    /// Имя кода в документации.
    pub fn as_str(self) -> &'static str {
        match self {
            ReasonCode::Timeout => "TIMEOUT",
            ReasonCode::UnsupportedFormat => "UNSUPPORTED_FORMAT",
            ReasonCode::ContentEncrypted => "CONTENT_ENCRYPTED",
            ReasonCode::ResourceExhausted => "RESOURCE_EXHAUSTED",
            ReasonCode::ParseError => "PARSE_ERROR",
            ReasonCode::EngineError => "ENGINE_ERROR",
            ReasonCode::PolicyAbsent => "POLICY_ABSENT",
            ReasonCode::PolicyInvalid => "POLICY_INVALID",
            ReasonCode::PolicyExpired => "POLICY_EXPIRED",
            ReasonCode::ContentChanged => "CONTENT_CHANGED",
        }
    }

    /// Причины, которые может сообщать инспекция (не состояние политики).
    pub fn is_inspection_reason(self) -> bool {
        !matches!(self, ReasonCode::PolicyAbsent | ReasonCode::PolicyInvalid | ReasonCode::PolicyExpired)
    }
}
