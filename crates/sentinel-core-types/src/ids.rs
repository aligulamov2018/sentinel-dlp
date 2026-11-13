//! Непрозрачные идентификаторы.
//!
//! Ядро идентификаторы не интерпретирует, только сравнивает (SENTINEL_CORE_DESIGN.md §3).
//! Каналы, операции и классы файлов в документации не заданы как закрытый перечень
//! идентификаторов (CAPABILITY_MATRIX.md §2 описывает их текстом), поэтому здесь они тоже
//! непрозрачные строки. Порядок — байтовый (`Ord` для `String`), как требует §6.3.

use std::fmt;

macro_rules! opaque_id {
    ($(#[$m:meta])* $name:ident) => {
        $(#[$m])*
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            /// Создаёт идентификатор. Пустая строка допускается типом; содержательные
            /// ограничения проверяет загрузчик бандла.
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// Значение идентификатора.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({:?})", stringify!($name), self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

opaque_id!(
    /// Идентификатор правила (`Rule.rule_id`).
    RuleId
);
opaque_id!(
    /// Идентификатор записи бандла уровня политики: `IncompleteInspectionAction`,
    /// `PartInspectionExclusion`, `PrecedenceConflictApproval`, `CapabilityException`.
    RecordId
);
opaque_id!(
    /// Идентификатор детектора.
    DetectorId
);
opaque_id!(
    /// Идентификатор бандла (`policy_bundle_id`).
    BundleId
);
opaque_id!(
    /// Канал (CAPABILITY_MATRIX.md §2).
    Channel
);
opaque_id!(
    /// Операция (CAPABILITY_MATRIX.md §2).
    Operation
);
opaque_id!(
    /// Класс файла по сигнатуре (§4, этап identify).
    FileClass
);
opaque_id!(
    /// Идентификатор группы; членство разрешает хост.
    GroupId
);
opaque_id!(
    /// Идентификатор подразделения; членство разрешает хост.
    OrgUnitId
);
opaque_id!(
    /// Непрозрачная ссылка на субъекта.
    PrincipalRef
);
opaque_id!(
    /// Непрозрачная ссылка на устройство.
    DeviceRef
);
