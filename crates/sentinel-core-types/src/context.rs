//! Контекст решения (SENTINEL_CORE_DESIGN.md §3).

use crate::ids::{Channel, DeviceRef, GroupId, Operation, OrgUnitId, PrincipalRef};

/// Членство субъекта; разрешает хост, ядро только сравнивает.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeAttr {
    Group(GroupId),
    OrgUnit(OrgUnitId),
}

/// Действительность политики по времени; вычисляет хост (FAILURE_MODE_DESIGN.md §4.6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolicyValidity {
    Current,
    Stale,
    TimeUnverifiable,
    Expired,
}

/// Контекст одного решения.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionContext {
    pub channel: Channel,
    pub operation: Operation,
    pub principal_ref: PrincipalRef,
    pub device_ref: DeviceRef,
    pub scope_attrs: Vec<ScopeAttr>,
    pub policy_validity: PolicyValidity,
    /// Время решения для условий по времени (Unix-секунды). Ядро часов не читает. Атомов по
    /// времени в наборе §6.1 нет, поэтому PDP этапа 1 поле только переносит.
    pub decision_time: Option<i64>,
}
