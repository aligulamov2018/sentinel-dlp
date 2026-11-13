//! Действительность разрешающих отступлений (SENTINEL_CORE_DESIGN.md §3, §6.8).
//!
//! Вычисляет хост на момент вызова, которому передаётся вход (INV-H3); ядро часов не читает.
//! Ядро только нормализует вход и обрабатывает отсутствующий и противоречивый вход (§6.8).

use crate::ids::RecordId;

/// Состояние времени хоста на момент вычисления входа.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeState {
    Current,
    Stale,
    TimeUnverifiable,
}

/// Вход действительности отступлений.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeviationValidity {
    /// Хост передал действительные на момент решения записи.
    Provided { time_state: TimeState, valid_ids: Vec<RecordId> },
    /// Вход не получен: граница сериализации, сбой хоста.
    Unavailable,
}
