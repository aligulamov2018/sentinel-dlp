# Sentinel DLP — Data Model

| Атрибут | Значение |
| --- | --- |
| Назначение | Логическая модель данных: классы данных, сущности, схема события с классификацией полей |
| Версия | 0.4.1 (исправления по ревью AR-01…AR-08, SR2-01…SR2-04, SR3-01…SR3-03, SR4-01) |
| Дата | 2026-11-11 |
| Статус | Proposed — логическая модель; физическая схема и миграции не созданы |
| Решения | ADR-0005, ADR-0006, ADR-0017, D-06, D-07 |
| Связанные документы | docs/architecture/EVENT_PIPELINE_DESIGN.md, docs/security/DATA_PROTECTION_DESIGN.md, docs/SYSTEM_ARCHITECTURE.md |

## 1. Классы данных

| Класс | Сущности | Хранилище | Требования |
| --- | --- | --- | --- |
| Конфигурация | `org_unit`, `principal`, `endpoint`, `agent_identity`, `agent_epoch`, `device_rule`, `data_identifier`, `dictionary`, `edm_index`, `fingerprint_set`, `policy`, `policy_version`, `role`, `permission`, `role_binding` | PostgreSQL | Версионирование, история, двойное подтверждение публикации политик |
| Телеметрия | `event`, `event_receipt`, `endpoint_seq_state`, `loss_declaration`, `loss_recovery`, `event_integrity_conflict`, `seq_state_history` | PostgreSQL; `event` секционирована по `received_at` | Сериализованный идемпотентный приём (EVENT_PIPELINE_DESIGN.md §4) |
| Асинхронная обработка | `outbox` (`state`, `lease_owner`, `lease_token`, `lease_until`, `idempotency_key`) | PostgreSQL | Атомарный захват, fencing по `lease_token` (EVENT_PIPELINE_DESIGN.md §5) |
| Инциденты | `incident`, `incident_event`, `assignment`, `comment`, `status_transition`, `legal_hold` | PostgreSQL | Область доступа, аудит просмотров |
| Evidence | `evidence` (метаданные, зашифрованный DEK, версия KEK периода), объект; `kek_period`; `deletion_tombstone` (вне резервируемой БД) | PostgreSQL + отдельное хранилище (C-05) | Шифрование, целостность, crypto-shredding (DATA_PROTECTION_DESIGN.md §7) |
| Аудит | `audit_record`, `audit_checkpoint` | PostgreSQL, только добавление | Цепочка хешей, внешнее закрепление |
| Исключения | `break_glass_grant`, `policy_exception` (кандидат FID/DLP-POLICY-009) | PostgreSQL | Срок, `max_offline_validity`, область, двойное подтверждение |
| Состояние агента (локально) | `time_state`, локальная очередь, fallback-таблица драйвера | Endpoint | Целостность (HMAC), но не свежесть; разрешающие отступления только в памяти сервиса (FAILURE_MODE_DESIGN.md §4.3–§4.5) |

## 2. Ключевые ограничения целостности

- `policy_version` неизменяема после публикации; откат создаёт новую версию.
- `event_receipt` — первичный ключ `(endpoint_id, agent_epoch, seq)`; таблица не секционирована по дате.
- Приём событий одного `(endpoint_id, agent_epoch)` сериализуется блокировкой строки `endpoint_seq_state`.
- Завершение и продление задания `outbox` возможны только с актуальным `lease_token` (условие на `state` и `lease_token`), в том числе после `lease_until`, если задание не перехвачено. Истечение аренды вычисляется по `clock_timestamp()` БД и только разрешает повторный захват. Бизнес-обработка не держит открытых транзакций (EVENT_PIPELINE_DESIGN.md §5.3).
- `loss_declaration`, `loss_recovery`, `event_integrity_conflict`, `seq_state_history` — только добавление: у роли приёма нет `UPDATE` и `DELETE`.
- `loss_declaration`:
  - уникален по `(endpoint_id, agent_epoch, marker_seq)`;
  - `declared_from = marker_seq <= declared_to <= marker_seq + MAX_LOSS_SPAN − 1`;
  - `status` ∈ {`APPLIED`, `REJECTED_INVALID`}; отказы `LIMIT` и `OUT_OF_WINDOW` не записываются;
  - замкнутые диапазоны `[marker_seq, declared_to]` деклараций `APPLIED` не пересекаются между собой, с квитанциями других `seq` и с записями `loss_recovery`.
- Фактически потерянные `seq` = диапазоны `APPLIED` минус восстановленные (EVENT_PIPELINE_DESIGN.md §4.2).
- `endpoint_seq_state`: `pending_count <= MAX_PENDING`; `max_seen <= low_watermark + MAX_SEQ_AHEAD`; `low_watermark` не убывает; `pending_count >= 0`; `last_batch_no` (диапазон `1 … 2^63 − 1`) растёт строго монотонно; `last_batch_no`, `last_batch_digest` и `last_batch_response` изменяются только вместе, в транзакции пакета; повтор передачи с тем же номером и тем же digest получает сохранённый ответ, с другим digest — `BATCH_INTEGRITY_CONFLICT` без изменений; закрытая эпоха (`closed_at`) приём не допускает (EVENT_PIPELINE_DESIGN.md §4.3, §4.5.3, §4.10).
- Событие с корректным конвертом и недопустимым содержимым сохраняется в `event_integrity_conflict` с причиной `SCHEMA_INVALID`, а не в `event` (EVENT_PIPELINE_DESIGN.md §4.4).
- `enforced_action = BLOCK` допустим только при `enforcement_status = ENFORCED`.
- `agent_epoch` уникальна для `endpoint_id`; новая эпоха регистрируется сервером.
- `audit_record.prev_hash` = хеш предыдущей записи; удаление и изменение записей через API невозможны.
- Модуль обращается только к своим таблицам (ADR-0001); у каждого модуля отдельная роль БД.

## 3. Схема события

Уровень — минимальный уровень передачи по ADR-0017, на котором поле присутствует. Чувствительность — классификация по DATA_PROTECTION_DESIGN.md §2.

| Поле | Тип | Уровень | Чувствительность | Назначение |
| --- | --- | --- | --- | --- |
| `endpoint_id` | UUID (из сертификата) | METADATA | Идентифицирует устройство | Идентичность |
| `agent_epoch` | u32 | METADATA | — | Дедупликация |
| `seq` | u64 | METADATA | — | Дедупликация, пропуски |
| `event_id` | UUIDv7 | METADATA | — | Ссылка |
| `class` | P1, P2, P3 | METADATA | — | Приоритет и квоты |
| `type` | перечисление | METADATA | — | Например, `DECISION_BLOCK`, `POLICY_REJECTED`, `EVENTS_LOST` |
| `agent_wall_ts`, `agent_mono_ns`, `boot_id`, `clock_offset_est` | время | METADATA | — | Время (EVENT_PIPELINE_DESIGN.md §6) |
| `received_at` | время сервера | — | — | Секционирование, хранение |
| `principal` | идентификатор каталога | METADATA | Персональные данные | Политика, расследование |
| `channel`, `operation` | перечисление | METADATA | — | Контекст |
| `object.path_class`, `object.extension` | перечисление, строка | METADATA | Низкая | Политика, статистика |
| `object.path_hmac` | HMAC | METADATA (`path_level` ≥ PSEUDONYMIZED) | Псевдоним | Корреляция |
| `object.path_full` | строка, зашифрована | Только `path_level = FULL` | Может быть идентифицирующей | Расследование, при праве `incident.view_path` |
| `object.sha256`, `object.size`, `object.true_type` | — | METADATA | Хеш идентифицирует документ | Корреляция, кэш |
| `destination` | том, устройство, получатель | METADATA | Может быть идентифицирующей | Контекст |
| `decision.verdict`, `decision.reasons[]`, `decision.fallback_applied` | перечисления | METADATA | — | Контракт D-05 |
| `required_action`, `enforced_action`, `enforcement_status` | перечисления | METADATA | — | Разделение требуемого действия, фактического действия и результата исполнения (CAPABILITY_MATRIX.md §4.1; FAILURE_MODE_DESIGN.md §2.4) |
| `channel_control_state` | CONTROLLED, DEGRADED, UNCONTROLLED, UNKNOWN | METADATA | — | Состояние контроля канала (в heartbeat и событиях `CONTROL_LOST`, `CONTROL_RESTORED`) |
| `after_loss_declaration` | bool | — | — | Позднее событие из объявленно потерянного диапазона; связано с записью `loss_recovery` |
| `decision.policy_version`, `decision.matched_rules[]` | — | METADATA | — | Объяснимость |
| `inspection.status`, `inspection.detectors[]`, `inspection.match_counts` | — | METADATA | Раскрывает тип данных | Объяснимость |
| `inspection.masked_snippets[]` | строки | MASKED_MATCH | Частичное содержимое | Проверка срабатываний |
| `evidence_ref` | ссылка | FULL_EVIDENCE | — | Связь с evidence |
| `payload_hash` | SHA-256 переданных байтов тела, без разбора схемы (EVENT_PIPELINE_DESIGN.md §2) | — | — | Обнаружение конфликтов целостности; проверка конверта |

## 4. Открытые вопросы модели

- Физическая схема, индексы и роли БД — после согласования SYSTEM_ARCHITECTURE.md.
- Хранение EDM-индексов и защита от перебора (T-23).
- Мультиарендность — кандидат вопроса из FID/OQ-08.
