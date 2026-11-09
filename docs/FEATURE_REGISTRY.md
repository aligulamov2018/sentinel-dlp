# Sentinel DLP — Feature Registry

| Атрибут | Значение |
|---|---|
| Статус документа | Draft — ожидает согласования |
| Версия | 0.1 |
| Дата | 2026-11-09 |
| Основание | `CLAUDE.md`, разделы 4, 8, 11 |
| Требования | [PRODUCT_REQUIREMENTS.md](PRODUCT_REQUIREMENTS.md), раздел 7 |
| Архитектура | [SYSTEM_ARCHITECTURE.md](SYSTEM_ARCHITECTURE.md) |

**Фактическое состояние: реализованных функций нет.** Все 106 функций имеют статус `Proposed`, поскольку архитектура и требования не согласованы. Тестов не существует; API и модель данных в карточках являются проектными намерениями и будут уточнены в `API_SPECIFICATION.md` и `DATA_MODEL.md`.

## Правила ведения реестра

- **Идентификаторы** не используются повторно. Функция, которая больше не нужна, переводится в `Deprecated`, а не удаляется.
- **Префиксы.** Помимо перечисленных в `CLAUDE.md`, добавлены `DLP-DISCOVERY` и `DLP-CONSOLE`, поскольку это самостоятельные подсистемы из разделов 4.5 и 4.7.
- **Функции сверх списка `CLAUDE.md`.** `DLP-POLICY-014` и `DLP-POLICY-015` вынесены отдельно, поскольку определяют требование к объяснимости и безопасному внедрению; `DLP-CORE-*` раскрывает обязательные механизмы раздела 5.
- **Изменение статуса** требует доказательства: ссылки на код и тесты. Наличие интерфейса не означает, что функция реализована.

| Статус | Условие перехода |
|---|---|
| `Proposed` | Функция описана; требования не согласованы |
| `Designed` | Утверждены требования, архитектура и критерии приёмки |
| `In Progress` | Идёт реализация по утверждённому проекту |
| `Implemented` | Код реализован и прошёл обязательную проверку; тесты ещё не полны |
| `Tested` | Обязательные тесты выполнены, результаты в `TEST_RESULTS.md` |
| `Integrated` | Функция работает в сквозном сценарии с реальными компонентами |
| `Production Ready` | Пройдены эксплуатационные и приёмочные испытания |
| `Deprecated` | Функция выведена из использования |

**Приоритеты:** P0 — MVP (R0.1–R0.4); P1 — полнота продукта (R0.5–R0.7); P2 — R1.0; P3 — после R1.0. Для функций конечных точек поле релиза указывает первый выпуск на Windows. Поддержка Linux начинается в R0.5, macOS — в R0.7 и подтверждается матрицей возможностей.

## Сводка

| Подсистема | Префикс | P0 | P1 | P2 | P3 | Всего |
|---|---|---:|---:|---:|---:|---:|
| Платформа и безопасность ядра | `DLP-CORE` | 12 | 0 | 0 | 0 | 12 |
| Endpoint DLP | `DLP-ENDPOINT` | 10 | 3 | 0 | 0 | 13 |
| Content Inspection Engine | `DLP-DETECTION` | 8 | 4 | 1 | 1 | 14 |
| Policy Engine | `DLP-POLICY` | 14 | 1 | 0 | 0 | 15 |
| Network DLP | `DLP-NETWORK` | 6 | 2 | 1 | 0 | 9 |
| Data Discovery | `DLP-DISCOVERY` | 0 | 8 | 0 | 0 | 8 |
| Incident Management | `DLP-INCIDENT` | 7 | 3 | 1 | 0 | 11 |
| Management Console | `DLP-CONSOLE` | 8 | 4 | 0 | 0 | 12 |
| Enterprise Integrations | `DLP-INTEGRATION` | 5 | 5 | 1 | 1 | 12 |
| **Итого** | | **70** | **30** | **4** | **2** | **106** |

## Карточки функций

### Платформа и безопасность ядра

#### DLP-CORE-001 — Operator Authentication & MFA

- **Description:** Аутентифицировать операторов консоли и API, требовать второй фактор для административных операций, управлять временем жизни сессий.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-001
- **Responsible Component:** cp-identity
- **Dependencies:** DLP-CORE-005, DLP-CORE-003
- **API:** REST /api/v1/auth
- **Data Model:** operator, operator_credential, session
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Административная операция без пройденного второго фактора отклоняется; пароли хранятся только как Argon2id-хеш; вход, отказ и отзыв сессии попадают в аудит.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Федеративный вход появляется в DLP-INTEGRATION-003/004. Набор факторов (TOTP, WebAuthn) требует согласования.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CORE-002 — RBAC & Separation of Duties

- **Description:** Разграничивать доступ на уровне API по ролям и областям, разделять администрирование платформы, управление политиками, расследование и аудит.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-002
- **Responsible Component:** cp-identity
- **Dependencies:** DLP-CORE-001
- **API:** REST /api/v1/rbac
- **Data Model:** role, permission, role_binding, scope
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Каждый маршрут API покрыт тестом на запрет доступа; проверка прав выполняется на сервере; роль аудитора не может изменять данные; смена ролей попадает в аудит.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Многопользовательская изоляция арендаторов (multi-tenancy) не входит в область до решения OQ-08.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CORE-003 — Tamper-Evident Audit Log

- **Description:** Записывать действия операторов и системы в журнал только для добавления с хеш-цепочкой и подписанными контрольными точками.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-003
- **Responsible Component:** cp-audit
- **Dependencies:** DLP-CORE-005
- **API:** REST /api/v1/audit
- **Data Model:** audit_event, audit_checkpoint
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Изменение или удаление записи обнаруживается верификатором; действие не считается успешным, если запись аудита не сохранена; API не предоставляет изменение журнала.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Цепочка выявляет, но не предотвращает вмешательство администратора БД; для независимой гарантии нужен внешний экспорт (DLP-INTEGRATION-005).
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CORE-004 — PKI & Certificate Lifecycle

- **Description:** Выпускать, ротировать и отзывать сертификаты сервисов и агентов, регистрировать агента по одноразовому токену с ограниченным сроком.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-004
- **Responsible Component:** cp-agent, cp-platform
- **Dependencies:** DLP-CORE-005
- **API:** gRPC AgentEnrollment; REST /api/v1/pki
- **Data Model:** ca_config, agent_certificate, enrollment_token, revocation
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Токен нельзя использовать повторно; агент с отозванным сертификатом теряет доступ в пределах согласованного окна; ротация проходит без потери событий.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Привязка ключа к TPM/Secure Enclave зависит от оборудования. Режим с внешним корпоративным CA требует проверки (PoC-07).
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CORE-005 — Secret Management

- **Description:** Получать секреты из подключаемого хранилища, исключить их появление в логах, репозитории и образах контейнеров.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-005
- **Responsible Component:** cp-platform
- **Dependencies:** нет
- **API:** Внутренний интерфейс SecretProvider
- **Data Model:** secret_reference
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Сканер секретов в CI не находит совпадений; запуск без доступа к секрету завершается контролируемой ошибкой; ротация секрета не требует пересборки.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Перечень поддерживаемых хранилищ (Vault-совместимое, файл с ограниченными правами, HSM) не утверждён.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CORE-006 — Encryption at Rest

- **Description:** Шифровать доказательства, чувствительные поля БД и данные агента на диске конвертным шифрованием с управляемой ротацией ключей.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-006
- **Responsible Component:** cp-platform, cp-incident
- **Dependencies:** DLP-CORE-005
- **API:** Нет внешнего API
- **Data Model:** key_metadata, wrapped_data_key
- **Implementation Status:** Proposed
- **Acceptance Criteria:** В объектном хранилище и дампе БД отсутствует открытый текст тестовых маркеров; ротация ключа KEK не требует перешифровки данных; потеря KEK описана в процедуре восстановления.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Локальный ключ агента защищён только в пределах возможностей ОС; привилегированный локальный администратор остаётся в модели угроз.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CORE-007 — Configuration Management

- **Description:** Загружать и проверять конфигурацию по схеме, версионировать изменения и отказывать в запуске при небезопасных значениях.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-007
- **Responsible Component:** cp-platform
- **Dependencies:** DLP-CORE-005
- **API:** REST /api/v1/system/config
- **Data Model:** system_config, config_revision
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Невалидная конфигурация останавливает запуск с понятной ошибкой; TLS нельзя отключить в производственном профиле; изменение конфигурации попадает в аудит.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Горячее применение поддерживается не для всех параметров; перечень определяется при проектировании.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CORE-008 — Observability & Health

- **Description:** Выводить структурированные логи с корреляционным идентификатором, метрики и проверки готовности каждого компонента.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-008
- **Responsible Component:** cp-platform
- **Dependencies:** нет
- **API:** /healthz, /readyz, /metrics; REST /api/v1/system/health
- **Data Model:** метрики вне БД
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Запрос прослеживается по идентификатору через все компоненты; содержимое инспектируемых данных не попадает в логи (проверяется тестом с маркером).
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Стек сбора логов и метрик предоставляется заказчиком; поставка в комплекте требует отдельного решения.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CORE-009 — Migrations, Backup & Restore

- **Description:** Применять версионированные миграции схемы и обеспечивать проверяемое резервное копирование и восстановление.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-009
- **Responsible Component:** cp-platform
- **Dependencies:** DLP-CORE-006
- **API:** CLI sentinelctl
- **Data Model:** schema_migration
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Обновление с предыдущего релиза проверяется в CI; восстановление из копии проходит регулярный автоматический тест; прерванная миграция не оставляет неконсистентную схему.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Откат миграций с необратимым преобразованием данных выполняется только через восстановление из копии.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CORE-010 — On-Premises & Air-Gapped Packaging

- **Description:** Поставлять OCI-образы и установочные артефакты для on-premises контура, включая установку без доступа в интернет.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-010
- **Responsible Component:** release-pipeline
- **Dependencies:** DLP-CORE-011
- **API:** Нет
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Чистая установка и обновление выполняются в изолированном от интернета стенде; компоненты не обращаются к внешним адресам (проверяется перехватом трафика).
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Целевые версии Kubernetes/OpenShift и ОС хостов требуют утверждения (OQ-05).
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CORE-011 — Supply Chain Security

- **Description:** Подписывать артефакты выпуска, формировать SBOM и блокировать выпуск при неустранённых критических уязвимостях зависимостей.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-011
- **Responsible Component:** release-pipeline
- **Dependencies:** нет
- **API:** Нет
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Для каждого артефакта есть подпись и SBOM; установка неподписанного артефакта отклоняется; CI падает при нарушении порога уязвимостей или запрещённой лицензии.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Подпись драйверов и системных расширений зависит от внешних программ Microsoft и Apple (RSK-01, RSK-02).
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CORE-012 — Versioned Platform API

- **Description:** Предоставлять версионированный REST API со спецификацией OpenAPI, валидацией ввода, ограничением частоты запросов и единым форматом ошибок.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-CORE-012
- **Responsible Component:** cp-platform
- **Dependencies:** DLP-CORE-001, DLP-CORE-002
- **API:** REST /api/v1 (OpenAPI)
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Контрактные тесты сверяют реализацию со спецификацией; ломающее изменение в рамках версии обнаруживается в CI; невалидный ввод отклоняется до бизнес-логики.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Политика сроков поддержки версий API ещё не определена.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

### Endpoint DLP

#### DLP-ENDPOINT-001 — File Activity Monitoring

- **Description:** Регистрировать создание, чтение, запись, переименование и удаление файлов с контекстом пользователя, процесса и тома.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-EP-001
- **Responsible Component:** agent-sensor-win, agent-sensor-linux, agent-sensor-macos
- **Dependencies:** DLP-ENDPOINT-009, DLP-CORE-004
- **API:** gRPC AgentEvents
- **Data Model:** event, agent
- **Implementation Status:** Proposed
- **Acceptance Criteria:** На эталонной нагрузке события соответствуют эталонному журналу операций без пропусков, кроме явно учтённых счётчиком потерь; накладные расходы укладываются в NFR-PERF-01.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Windows требует подписанный minifilter; Linux и macOS подтверждаются PoC-02 и PoC-03. Отключённый агент событий не формирует.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-002 — USB Device Control

- **Description:** Разрешать и запрещать подключение USB-устройств по классу, VID/PID и серийному номеру.
- **Priority:** P0 (релиз R0.4)
- **Functional Requirements:** FR-EP-002
- **Responsible Component:** agent-sensor-win, agent-sensor-linux, agent-sensor-macos
- **Dependencies:** DLP-ENDPOINT-007, DLP-POLICY-004
- **API:** gRPC AgentControl; REST /api/v1/policies
- **Data Model:** device_rule, device_event
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Запрещённый класс устройства недоступен пользователю; разрешённый по белому списку работает; решение объяснимо и записано как событие.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** VID/PID и серийный номер подделываются аппаратно. Механизм для каждой ОС подтверждается отдельно (матрица в SYSTEM_ARCHITECTURE.md).
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-003 — Removable Media Control

- **Description:** Применять к съёмным носителям режимы запрета, только чтения и записи с инспекцией содержимого.
- **Priority:** P0 (релиз R0.4)
- **Functional Requirements:** FR-EP-003
- **Responsible Component:** agent-sensor-win, agent-sensor-linux, agent-sensor-macos
- **Dependencies:** DLP-ENDPOINT-002, DLP-ENDPOINT-007
- **API:** gRPC AgentControl
- **Data Model:** device_rule, event
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Запись файла с тестовым маркером на носитель в режиме инспекции блокируется, файл без маркера записывается; режим только чтения не допускает записи ни через один проверенный API.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Linux не имеет разрешения на запись в fanotify; контроль записи на Linux может быть ограничен (PoC-02). Прямой доступ к тому требует отдельной проверки.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-004 — Print Monitoring

- **Description:** Регистрировать задания печати: пользователь, принтер, документ, количество страниц, с блокировкой там, где механизм подтверждён.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-EP-004
- **Responsible Component:** agent-sensor-win, agent-sensor-linux, agent-sensor-macos
- **Dependencies:** DLP-ENDPOINT-007
- **API:** gRPC AgentEvents
- **Data Model:** event
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Задание с тестового приложения фиксируется с корректными атрибутами на Windows при включённом и выключенном Windows protected print mode; поведение блокировки подтверждено или явно отсутствует.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** В Windows protected print mode сторонние модули спулера запрещены; инспекция содержимого и блокировка задания не гарантированы (PoC-05).
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-005 — Clipboard Monitoring

- **Description:** Обнаруживать помещение чувствительных данных в буфер обмена в пользовательском сеансе и регистрировать событие.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-EP-005
- **Responsible Component:** agent-session-helper
- **Dependencies:** DLP-ENDPOINT-007, DLP-DETECTION-001
- **API:** gRPC AgentEvents
- **Data Model:** event
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Изменение буфера с тестовым маркером создаёт событие с источником и без сохранения полного содержимого; отсутствие хелпера в сеансе отражается в состоянии агента.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Уведомление Windows не даёт предотвратить операцию; блокировка вставки требует отдельного механизма. Wayland и macOS ограничивают доступ (PoC-06).
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-006 — File Transfer Control

- **Description:** Контролировать копирование файлов на сетевые ресурсы и передачу через заданные приложения и каналы.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-EP-006
- **Responsible Component:** agent-core, agent-sensor-win, agent-sensor-linux
- **Dependencies:** DLP-ENDPOINT-001, DLP-ENDPOINT-007, DLP-DETECTION-013
- **API:** gRPC AgentControl
- **Data Model:** event, policy_version
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Копирование файла с маркером на SMB-ресурс блокируется политикой; решение содержит процесс, назначение и сработавшее правило.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Загрузка из браузера и зашифрованные протоколы приложений не инспектируются без расширения браузера или прокси; перечень покрытых сценариев публикуется явно.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-007 — Endpoint Policy Enforcement

- **Description:** Выполнять решение политики на конечной точке (разрешить, предупредить, запросить обоснование, заблокировать) с уведомлением пользователя.
- **Priority:** P0 (релиз R0.4)
- **Functional Requirements:** FR-EP-007
- **Responsible Component:** agent-core
- **Dependencies:** DLP-ENDPOINT-008, DLP-POLICY-013, DLP-DETECTION-013
- **API:** gRPC AgentControl
- **Data Model:** policy_bundle, decision
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Одинаковый вход даёт одинаковое решение с тем же пакетом политик; при таймауте инспекции применяется заданный режим fail-open/fail-closed; каждое решение содержит ссылку на правило.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Блокировка возможна только в каналах с подтверждённым механизмом; остальные каналы работают в режиме наблюдения.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-008 — Local Policy Cache

- **Description:** Хранить на агенте подписанный пакет политик и применять только пакет с проверенной подписью и версией не ниже текущей.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-EP-008
- **Responsible Component:** agent-core
- **Dependencies:** DLP-POLICY-010, DLP-CORE-006
- **API:** gRPC AgentControl
- **Data Model:** policy_bundle
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Подменённый или более старый пакет отклоняется с событием; после перезапуска без сети применяется последний проверенный пакет.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Срок действия пакета при длительном отсутствии связи определяется политикой организации.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-009 — Offline Event Buffering

- **Description:** Сохранять события в зашифрованном локальном журнале и доставлять их после восстановления связи без дублей на сервере.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-EP-009
- **Responsible Component:** agent-core
- **Dependencies:** DLP-CORE-006
- **API:** gRPC AgentEvents
- **Data Model:** локальный журнал агента
- **Implementation Status:** Proposed
- **Acceptance Criteria:** После обрыва связи и перезапуска все события доставляются и дедуплицируются; при заполнении квоты применяется заданная политика вытеснения с учётом потерь.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Объём буфера ограничен квотой; при превышении потери неизбежны, но фиксируются.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-010 — Agent Health Monitoring

- **Description:** Передавать состояние агента: версия, активность сенсоров, версия политики, очередь, потери, потребление ресурсов.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-EP-010
- **Responsible Component:** agent-core, cp-agent
- **Dependencies:** DLP-CORE-004
- **API:** gRPC AgentControl; REST /api/v1/agents
- **Data Model:** agent, agent_status
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Отключение сенсора и потеря связи отражаются в консоли в пределах NFR-OPS-03; статус не показывается как здоровый при отсутствии свежего heartbeat.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Heartbeat не подтверждает работоспособность сенсора полностью; нужны синтетические проверки.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-011 — Agent Integrity Verification

- **Description:** Проверять подпись компонентов и конфигурации агента, защищать от остановки в пределах возможностей ОС и сообщать о вмешательстве.
- **Priority:** P0 (релиз R0.4)
- **Functional Requirements:** FR-EP-011
- **Responsible Component:** agent-watchdog
- **Dependencies:** DLP-CORE-011
- **API:** gRPC AgentControl
- **Data Model:** agent_status, event
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Подмена бинарного файла или конфигурации обнаруживается и создаёт событие; остановка службы непривилегированным пользователем невозможна.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Локальный администратор и root могут обойти защиту; полная защита от привилегированного пользователя не заявляется. Antimalware-PPL требует отдельной оценки.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-012 — Secure Agent Updates

- **Description:** Обновлять агента только подписанными пакетами поэтапно, с автоматическим откатом и остановкой раскатки при деградации.
- **Priority:** P0 (релиз R0.4)
- **Functional Requirements:** FR-EP-012
- **Responsible Component:** agent-updater, cp-agent
- **Dependencies:** DLP-CORE-011, DLP-ENDPOINT-013
- **API:** gRPC AgentControl; REST /api/v1/agents/updates
- **Data Model:** agent_release, rollout
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Неподписанный или более старый пакет отклоняется; сбой после обновления приводит к автоматическому откату; раскатка останавливается при превышении порога отказов.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Обновление драйвера может требовать перезагрузки и повторной подписи Microsoft.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-ENDPOINT-013 — Centralized Agent Management

- **Description:** Вести реестр агентов, их группы и состояние, выполнять регистрацию, вывод из эксплуатации и назначение политик.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-EP-013
- **Responsible Component:** cp-agent
- **Dependencies:** DLP-CORE-004
- **API:** REST /api/v1/agents
- **Data Model:** agent, agent_group
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Операции над агентом доступны только через авторизованный API и попадают в аудит; выведенный агент теряет доступ.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Массовая установка выполняется средствами заказчика (GPO, SCCM, MDM); собственного механизма развёртывания нет.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

### Content Inspection Engine

#### DLP-DETECTION-001 — Regular Expressions

- **Description:** Находить совпадения с регулярными выражениями в извлечённом тексте с гарантией времени выполнения, линейного по длине входа.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-DET-001
- **Responsible Component:** libinspect
- **Dependencies:** нет
- **API:** REST /api/v1/detectors; gRPC Inspection
- **Data Model:** detector
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Патологические выражения не вызывают экспоненциального времени (тест ReDoS); валидаторы (например, контрольные суммы) применяются к совпадениям; точность измеряется на корпусе.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Движок с линейным временем не поддерживает обратные ссылки и lookaround; недоступные конструкции отклоняются при сохранении правила.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-002 — Exact Data Matching

- **Description:** Сопоставлять содержимое с солёными хешами значений из структурированных источников без хранения исходных данных в индексе.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DET-002
- **Responsible Component:** libinspect, cp-policy
- **Dependencies:** DLP-DETECTION-001, DLP-CORE-006
- **API:** REST /api/v1/detectors/edm
- **Data Model:** edm_index
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Индекс не содержит открытых значений; совпадение по нескольким полям одной записи обнаруживается; обновление индекса не прерывает инспекцию.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Значения с малой энтропией подвержены перебору; размер индекса на агенте ограничен, часть проверок может выполняться только на сервере.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-003 — Document Fingerprinting

- **Description:** Распознавать документы и их производные по отпечаткам нормализованного содержимого.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DET-003
- **Responsible Component:** libinspect, cp-policy
- **Dependencies:** DLP-DETECTION-007, DLP-DETECTION-008
- **API:** REST /api/v1/detectors/fingerprints
- **Data Model:** fingerprint_index
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Изменённая копия эталонного документа обнаруживается выше заданного порога на тестовом корпусе; точность и полнота публикуются в TEST_RESULTS.md.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Перевод, глубокое перефразирование и снимки экрана не обнаруживаются.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-004 — Partial Document Matching

- **Description:** Обнаруживать вставку фрагментов защищаемых документов в другие документы и сообщения.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DET-004
- **Responsible Component:** libinspect
- **Dependencies:** DLP-DETECTION-003
- **API:** REST /api/v1/detectors/fingerprints
- **Data Model:** fingerprint_index
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Фрагмент эталонного документа выше минимальной длины обнаруживается и связывается с источником; порог длины настраивается.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Короткие фрагменты не обнаруживаются; ложные срабатывания на типовых шаблонах подавляются исключениями.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-005 — Metadata Inspection

- **Description:** Проверять метаданные файлов и сообщений: авторов, свойства документов, метки классификации, размер и имя.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-DET-005
- **Responsible Component:** libinspect
- **Dependencies:** DLP-DETECTION-006
- **API:** gRPC Inspection
- **Data Model:** detector
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Правило срабатывает на заданное свойство тестового документа; вредоносные или слишком большие метаданные не нарушают работу парсера.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Метаданные легко изменяются и не должны быть единственным признаком критичной блокировки.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-006 — File Type Detection

- **Description:** Определять истинный тип файла по содержимому независимо от расширения и различать зашифрованные контейнеры.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-DET-006
- **Responsible Component:** libinspect
- **Dependencies:** нет
- **API:** gRPC Inspection
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Файл с изменённым расширением определяется правильно на тестовом корпусе; неизвестный тип и шифрование возвращаются как явный результат, а не как отсутствие совпадений.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Полиглот-файлы могут определяться неоднозначно; перечень типов публикуется.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-007 — Office Document Parsing

- **Description:** Извлекать текст и встроенные объекты из форматов OOXML и устаревших форматов Office в изолированном процессе.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-DET-007
- **Responsible Component:** libinspect-parsers
- **Dependencies:** DLP-DETECTION-006
- **API:** gRPC Inspection
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Набор фаззинг-тестов не приводит к сбою вне песочницы; сбой парсера возвращает результат «не инспектировано», к которому применяется политика.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Защищённые паролем файлы не читаются; сложные встроенные объекты могут быть пропущены и отражаются в результате.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-008 — PDF Inspection

- **Description:** Извлекать текст и встроенные файлы из PDF в изолированном процессе с лимитами ресурсов.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-DET-008
- **Responsible Component:** libinspect-parsers
- **Dependencies:** DLP-DETECTION-006
- **API:** gRPC Inspection
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Вредоносный или чрезмерно вложенный PDF завершается по лимиту без влияния на сервис; полностью графический PDF помечается как требующий OCR.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Без OCR сканы не анализируются; извлечение текста зависит от кодировок и шрифтов.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-009 — OCR

- **Description:** Распознавать текст на изображениях и сканах при асинхронной инспекции на сервере.
- **Priority:** P2 (релиз R1.0)
- **Functional Requirements:** FR-DET-009
- **Responsible Component:** inspection-service
- **Dependencies:** DLP-DETECTION-008
- **API:** gRPC Inspection
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** На эталонном корпусе русского и английского текста публикуются измеренные показатели качества и пропускная способность; отказ OCR не влияет на прочие детекторы.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Ресурсоёмко; не применяется в синхронной блокировке на агенте. Качество зависит от изображения.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-010 — Archive Inspection

- **Description:** Рекурсивно инспектировать архивы с ограничением глубины, коэффициента распаковки и общего объёма.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-DET-010
- **Responsible Component:** libinspect-parsers
- **Dependencies:** DLP-DETECTION-006
- **API:** gRPC Inspection
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Zip-бомба останавливается лимитом и возвращает явный статус; зашифрованный архив определяется и обрабатывается политикой; path traversal не приводит к записи вне песочницы.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Зашифрованные архивы без ключа не анализируются; можно только применить политику к факту шифрования.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-011 — Keyword Dictionaries

- **Description:** Выполнять поиск по управляемым словарям с нормализацией регистра и Unicode, включая русский язык.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-DET-011
- **Responsible Component:** libinspect
- **Dependencies:** нет
- **API:** REST /api/v1/detectors/dictionaries
- **Data Model:** dictionary
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Словарь версионируется и попадает в пакет политик; поиск учитывает нормализацию на тестовом корпусе; пропускная способность измерена на словаре заданного размера.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Морфология русского языка требует отдельной реализации и не гарантируется на первом этапе.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-012 — Contextual Classification

- **Description:** Оценивать совпадения с учётом близких ключевых слов, структуры документа, количества и источника.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DET-012
- **Responsible Component:** libinspect
- **Dependencies:** DLP-DETECTION-001, DLP-DETECTION-011
- **API:** gRPC Inspection
- **Data Model:** detector
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Правило с контекстом снижает ложные срабатывания относительно правила без контекста на эталонном корпусе; результат объясним перечнем признаков.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Порогам требуется настройка на данных организации.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-013 — Combined Detection Rules

- **Description:** Объединять детекторы логическими операторами и порогами количества в детерминированное правило.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-DET-013
- **Responsible Component:** libinspect, cp-policy
- **Dependencies:** DLP-DETECTION-001, DLP-DETECTION-011
- **API:** REST /api/v1/detectors
- **Data Model:** detector, rule
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Таблица истинности правила проверяется автоматически; результат содержит вклад каждого детектора; небезопасные и слишком сложные правила отклоняются при сохранении.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Глубина и сложность выражений ограничены для предсказуемого времени выполнения.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DETECTION-014 — ML/NLP Classification

- **Description:** Опционально классифицировать документы ML/NLP-моделями как дополнительный сигнал асинхронной проверки.
- **Priority:** P3 (релиз Post-1.0)
- **Functional Requirements:** FR-DET-014
- **Responsible Component:** ml-classifier
- **Dependencies:** DLP-DETECTION-012
- **API:** gRPC Inspection
- **Data Model:** model_registry
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Отключение компонента не влияет на блокировки; модель работает локально без внешних сервисов; качество измерено на данных организации до включения.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Не участвует как единственное основание блокировки; недетерминирован и требует контроля дрейфа.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

### Policy Engine

#### DLP-POLICY-001 — Centralized Policy Authoring

- **Description:** Создавать и изменять политики централизованно по валидируемой декларативной схеме.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-POL-001
- **Responsible Component:** cp-policy
- **Dependencies:** DLP-CORE-002
- **API:** REST /api/v1/policies
- **Data Model:** policy, policy_version, rule
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Невалидная политика не сохраняется и возвращает точную причину; экспорт и импорт политики дают эквивалентный результат.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Формат языка правил утверждается в ADR-010.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-002 — Scope Management

- **Description:** Определять области применения политики по каналу, субъекту, устройству и данным.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-POL-002
- **Responsible Component:** cp-policy
- **Dependencies:** DLP-POLICY-001
- **API:** REST /api/v1/policies
- **Data Model:** policy_scope
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Вычисление области покрыто табличными тестами; политика вне области не влияет на решение и это видно в объяснении.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Составные области увеличивают размер пакета для агента.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-003 — User & Group Policies

- **Description:** Применять политики по пользователям и группам каталога.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-POL-003
- **Responsible Component:** cp-policy
- **Dependencies:** DLP-INTEGRATION-001, DLP-INTEGRATION-002
- **API:** REST /api/v1/policies
- **Data Model:** policy_scope, directory_user, directory_group
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Смена членства в группе применяется в согласованное время и отражается в версии снимка каталога; решение содержит использованные группы.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** На автономном агенте действует последний полученный снимок членства.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-004 — Device Policies

- **Description:** Применять политики к устройствам и группам устройств.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-POL-004
- **Responsible Component:** cp-policy
- **Dependencies:** DLP-ENDPOINT-013
- **API:** REST /api/v1/policies
- **Data Model:** policy_scope, agent_group
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Агент получает только относящиеся к нему правила; перемещение между группами изменяет набор правил.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Атрибуты устройства, сообщаемые агентом, не считаются полностью доверенными.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-005 — Channel Policies

- **Description:** Применять политики к каналам: файл, USB, печать, буфер обмена, email, веб, с учётом доступных действий канала.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-POL-005
- **Responsible Component:** cp-policy
- **Dependencies:** DLP-POLICY-001
- **API:** REST /api/v1/policies
- **Data Model:** policy_scope
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Сохранение действия, недоступного каналу или ОС, отклоняется либо явно понижается до наблюдения с предупреждением.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Возможности каналов различаются по ОС (матрица в SYSTEM_ARCHITECTURE.md).
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-006 — Contextual Rules

- **Description:** Учитывать контекст: приложение, время, расположение сети, назначение, метку файла.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-POL-006
- **Responsible Component:** cp-policy
- **Dependencies:** DLP-POLICY-002
- **API:** REST /api/v1/policies
- **Data Model:** rule
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Условия контекста детерминированно проверяются табличными тестами; отсутствие атрибута обрабатывается заданным образом, а не ошибкой.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Контекст, предоставляемый агентом, может быть подменён привилегированным пользователем.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-007 — Policy Priorities

- **Description:** Назначать политикам явные уникальные приоритеты для предсказуемой оценки.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-POL-007
- **Responsible Component:** cp-policy
- **Dependencies:** DLP-POLICY-001
- **API:** REST /api/v1/policies
- **Data Model:** policy
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Одинаковый вход и набор политик всегда дают один результат независимо от порядка загрузки (тест на перестановках).
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Приоритеты требуют дисциплины администрирования при большом числе политик.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-008 — Conflict Resolution

- **Description:** Разрешать конфликты по опубликованному алгоритму: приоритет, затем наиболее строгое действие, затем идентификатор правила.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-POL-008
- **Responsible Component:** cp-policy
- **Dependencies:** DLP-POLICY-007
- **API:** REST /api/v1/policies/evaluate
- **Data Model:** policy, decision
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Каждое конфликтное сочетание из таблицы решений покрыто тестом; объяснение показывает проигравшие правила.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Алгоритм подлежит утверждению в ADR-010.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-009 — Policy Exceptions

- **Description:** Создавать ограниченные по области и сроку исключения с обоснованием и согласованием.
- **Priority:** P0 (релиз R0.4)
- **Functional Requirements:** FR-POL-009
- **Responsible Component:** cp-policy
- **Dependencies:** DLP-POLICY-008
- **API:** REST /api/v1/policies/exceptions
- **Data Model:** policy_exception
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Истёкшее исключение перестаёт действовать без участия оператора; создание и использование исключения попадают в аудит.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Истечение на автономном агенте зависит от локальных часов; отклонение часов учитывается в допуске.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-010 — Policy Versioning

- **Description:** Хранить неизменяемые версии политик и публиковать подписанные пакеты.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-POL-010
- **Responsible Component:** cp-policy
- **Dependencies:** DLP-POLICY-001
- **API:** REST /api/v1/policies/{id}/versions
- **Data Model:** policy_version, policy_bundle
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Опубликованная версия не изменяется; каждое решение и инцидент ссылаются на версию; пакет подписан, подпись проверяется получателем.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Хранение всех версий требует политики хранения.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-011 — Policy Change Audit

- **Description:** Фиксировать автора, время, причину и различия для всех изменений политик.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-POL-011
- **Responsible Component:** cp-policy, cp-audit
- **Dependencies:** DLP-CORE-003
- **API:** REST /api/v1/audit
- **Data Model:** audit_event
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Любое изменение политики без записи аудита не применяется; различия версий доступны в API.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Нет.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-012 — Policy Rollback

- **Description:** Откатывать политику к предыдущей версии созданием новой версии с прежним содержимым.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-POL-012
- **Responsible Component:** cp-policy
- **Dependencies:** DLP-POLICY-010
- **API:** REST /api/v1/policies/{id}/rollback
- **Data Model:** policy_version
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Откат не удаляет историю; агенты и шлюзы получают результат в пределах NFR-OPS-02; откат попадает в аудит.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Откат не отменяет уже принятые решения.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-013 — Offline Policy Enforcement

- **Description:** Применять политики без связи с сервером, используя только локальные данные пакета и локальные детекторы.
- **Priority:** P0 (релиз R0.4)
- **Functional Requirements:** FR-POL-013
- **Responsible Component:** agent-core
- **Dependencies:** DLP-ENDPOINT-008, DLP-POLICY-010
- **API:** Нет внешнего API
- **Data Model:** policy_bundle
- **Implementation Status:** Proposed
- **Acceptance Criteria:** При отключённой сети блокирующие правила продолжают работать; результат совпадает с онлайн-режимом для правил, не требующих серверных индексов.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Правила, требующие серверных индексов (крупный EDM), работают по заданному резервному действию.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-014 — Monitor Mode & Simulation

- **Description:** Вводить политику в режиме наблюдения и проверять её на тестовых образцах до включения блокировки.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-POL-014
- **Responsible Component:** cp-policy, libinspect
- **Dependencies:** DLP-POLICY-008
- **API:** REST /api/v1/policies/simulate
- **Data Model:** policy_version, decision
- **Implementation Status:** Proposed
- **Acceptance Criteria:** В режиме наблюдения операция не блокируется, но регистрируется решение, которое было бы принято; переход к блокировке требует отдельного действия и аудита.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Симуляция не полностью воспроизводит контекст конечной точки.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-POLICY-015 — Explainable Decisions

- **Description:** Возвращать для каждого решения версию политики, сработавшие правила и детекторы и причину действия без раскрытия полного содержимого.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-POL-015
- **Responsible Component:** cp-policy, libinspect
- **Dependencies:** DLP-POLICY-008
- **API:** REST /api/v1/policies/evaluate
- **Data Model:** decision
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Решение воспроизводимо по сохранённым входным признакам и версии политики; объяснение не содержит открытых чувствительных значений.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Объяснение ML-сигналов (после 1.0) ограничено признаками модели.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

### Network DLP

#### DLP-NETWORK-001 — Email DLP

- **Description:** Инспектировать тело письма, заголовки и вложения и применять действия: пропустить, отклонить, поместить в карантин (с R0.5).
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-NET-001
- **Responsible Component:** gw-smtp
- **Dependencies:** DLP-NETWORK-002, DLP-DETECTION-013, DLP-POLICY-008
- **API:** SMTP; REST /api/v1/incidents
- **Data Model:** event, incident
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Письмо с маркером во вложении отклоняется с заданным ответом; письмо без маркера доставляется без изменений и потерь; результат зафиксирован.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Содержимое S/MIME и PGP без ключей недоступно; применяется политика к факту шифрования.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-NETWORK-002 — SMTP/MTA Integration

- **Description:** Встраиваться в поток MTA как промежуточный SMTP-узел с поддержкой TLS и настраиваемым поведением при отказе.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-NET-002
- **Responsible Component:** gw-smtp
- **Dependencies:** нет
- **API:** SMTP
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Совместимость подтверждена с утверждёнными MTA; при отказе инспекции применяется заданный режим, письма не теряются.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Перечень MTA и вариант интеграции (next-hop или milter) подтверждаются в PoC-09.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-NETWORK-003 — HTTP(S) Proxy Integration

- **Description:** Принимать на инспекцию расшифрованный веб-трафик от существующего корпоративного прокси.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-NET-003
- **Responsible Component:** gw-icap
- **Dependencies:** DLP-NETWORK-004
- **API:** ICAP
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Загрузка с маркером через поддерживаемый прокси блокируется; проверка выполнена на утверждённых продуктах с публикацией версий.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Sentinel не расшифровывает TLS самостоятельно; контролируется только трафик, расшифрованный прокси с разрешения организации. Certificate pinning не обходится.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-NETWORK-004 — ICAP Integration

- **Description:** Реализовать ICAP-сервер с режимами REQMOD и RESPMOD, предпросмотром и ограничением ресурсов.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-NET-004
- **Responsible Component:** gw-icap
- **Dependencies:** DLP-DETECTION-013
- **API:** ICAP (RFC 3507)
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Контрактные тесты RFC 3507 проходят; большие и неполные тела не приводят к исчерпанию памяти; TLS для ICAP поддерживается там, где его поддерживает клиент.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Поддержка ICAPS различается между прокси.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-NETWORK-005 — File Upload Inspection

- **Description:** Извлекать и инспектировать файлы из запросов загрузки, включая multipart.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-NET-005
- **Responsible Component:** gw-icap, inspection-service
- **Dependencies:** DLP-NETWORK-004
- **API:** ICAP
- **Data Model:** event
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Multipart-загрузка с маркером блокируется; JSON/base64 и потоковые протоколы обрабатываются или явно отмечаются как не инспектируемые.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Закрытые протоколы и клиентское шифрование веб-приложений не инспектируются.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-NETWORK-006 — Cloud Application Controls

- **Description:** Применять правила к операциям в заданных облачных приложениях по профилям запросов.
- **Priority:** P2 (релиз R1.0)
- **Functional Requirements:** FR-NET-006
- **Responsible Component:** gw-icap
- **Dependencies:** DLP-NETWORK-003, DLP-POLICY-006
- **API:** ICAP; REST /api/v1/policies
- **Data Model:** cloud_app_catalog
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Для каждого профиля есть автоматизированный тест загрузки, а изменение веб-приложения обнаруживается регулярной проверкой профиля.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Профили требуют постоянной поддержки. API-интеграции с облачными сервисами (CASB) не входят в область.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-NETWORK-007 — Network Event Collection

- **Description:** Собирать события от сетевых компонентов и внешних источников в единую схему.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-NET-007
- **Responsible Component:** gw-icap, gw-smtp, event-ingest
- **Dependencies:** DLP-NETWORK-004
- **API:** gRPC EventIngest
- **Data Model:** event
- **Implementation Status:** Proposed
- **Acceptance Criteria:** События нормализуются без потерь на эталонной нагрузке; дубли устраняются; недоступность брокера не приводит к потере принятых событий.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Прослушивание трафика (SPAN/TAP) не входит в первую версию.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-NETWORK-008 — Policy-Based Blocking

- **Description:** Блокировать сетевую передачу на основании решений Policy Engine с заданным поведением при отказе.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-NET-008
- **Responsible Component:** gw-icap, gw-smtp
- **Dependencies:** DLP-POLICY-008
- **API:** ICAP; SMTP
- **Data Model:** decision
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Блокировка воспроизводится для каждого действия; истечение таймаута приводит к заданному режиму; режим отказа виден в логах и метриках.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Fail-open допускает утечку, fail-closed влияет на доступность; выбор за организацией.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-NETWORK-009 — Quarantine

- **Description:** Хранить задержанные письма и файлы в зашифрованном виде с разрешением или отклонением уполномоченным оператором.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-NET-009
- **Responsible Component:** cp-incident, gw-smtp
- **Dependencies:** DLP-CORE-006, DLP-INCIDENT-006
- **API:** REST /api/v1/quarantine
- **Data Model:** quarantine_item
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Освобождение и удаление требуют прав и попадают в аудит; содержимое зашифровано; истёкшие объекты обрабатываются по политике хранения.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Для веб-загрузок карантин, как правило, невозможен без изменения пользовательского сценария.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

### Data Discovery

#### DLP-DISCOVERY-001 — File Share Scanning

- **Description:** Сканировать файловые ресурсы SMB и NFS при ограничении нагрузки.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DISC-001
- **Responsible Component:** discovery-worker, cp-discovery
- **Dependencies:** DLP-DETECTION-013
- **API:** REST /api/v1/discovery
- **Data Model:** scan_job, scan_target
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Эталонный ресурс сканируется полностью с отчётом об ошибках доступа; ограничение скорости соблюдается; прерванное сканирование возобновляется.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Защищённые паролем и зашифрованные файлы отмечаются как не инспектированные.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DISCOVERY-002 — Sensitive Data Discovery

- **Description:** Находить чувствительные данные детекторами общего движка инспекции.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DISC-002
- **Responsible Component:** discovery-worker
- **Dependencies:** DLP-DISCOVERY-001
- **API:** REST /api/v1/discovery/results
- **Data Model:** scan_result
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Результаты на размеченном корпусе совпадают с результатами движка на агенте и шлюзе; значения маскируются.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Качество определяется настройкой детекторов.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DISCOVERY-003 — Data Classification

- **Description:** Присваивать обнаруженным данным метки классификации по настраиваемой схеме.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DISC-003
- **Responsible Component:** cp-discovery
- **Dependencies:** DLP-DISCOVERY-002
- **API:** REST /api/v1/classification
- **Data Model:** classification_label, scan_result
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Метка вычисляется детерминированно и объясняется сработавшими правилами; изменение схемы версионируется.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Запись меток в сами файлы в первую версию не входит.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DISCOVERY-004 — Metadata Analysis

- **Description:** Собирать владельца, права доступа, даты и размер обнаруженных файлов.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DISC-004
- **Responsible Component:** discovery-worker
- **Dependencies:** DLP-DISCOVERY-001
- **API:** REST /api/v1/discovery/results
- **Data Model:** scan_result
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Отчёт показывает избыточно доступные файлы на эталонном ресурсе; чтение метаданных не изменяет время доступа там, где ОС это позволяет.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Детальность прав зависит от протокола и учётной записи сканирования.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DISCOVERY-005 — Incremental Scanning

- **Description:** Повторно обрабатывать только изменившиеся файлы.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DISC-005
- **Responsible Component:** discovery-worker
- **Dependencies:** DLP-DISCOVERY-001
- **API:** REST /api/v1/discovery
- **Data Model:** scan_state
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Повторное сканирование неизменного ресурса не читает содержимое файлов; изменённый файл обрабатывается; изменение правил требует контролируемого пересканирования.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Ненадёжные метки времени на некоторых хранилищах требуют проверки хешем.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DISCOVERY-006 — Classification Reports

- **Description:** Формировать отчёты о размещении и классах чувствительных данных.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DISC-006
- **Responsible Component:** cp-reporting
- **Dependencies:** DLP-DISCOVERY-003
- **API:** REST /api/v1/reports
- **Data Model:** report
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Итоги отчёта сходятся с запросом к исходным данным (тест); экспорт не содержит открытых значений.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Набор шаблонов определяется с заказчиком.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DISCOVERY-007 — Scheduled Scans

- **Description:** Запускать сканирование по расписанию с окнами обслуживания.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DISC-007
- **Responsible Component:** cp-discovery
- **Dependencies:** DLP-DISCOVERY-001
- **API:** REST /api/v1/discovery/schedules
- **Data Model:** scan_schedule
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Задание стартует по расписанию один раз даже при нескольких экземплярах; вне окна запуск не выполняется.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Точность запуска зависит от доступности планировщика.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-DISCOVERY-008 — Access-Controlled Scanning

- **Description:** Выполнять сканирование под выделенными учётными записями с минимальными правами и разграничивать доступ к результатам.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-DISC-008
- **Responsible Component:** cp-discovery, discovery-worker
- **Dependencies:** DLP-CORE-005, DLP-CORE-002
- **API:** REST /api/v1/discovery/targets
- **Data Model:** scan_target
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Учётные данные не появляются в логах и API; пользователь без прав не видит результаты цели; все запуски попадают в аудит.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Широкий доступ на чтение учётной записи сканирования является риском и требует согласования.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

### Incident Management

#### DLP-INCIDENT-001 — Incident Registration

- **Description:** Создавать инцидент из решения политики идемпотентно, со ссылками на версию политики и источник.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-INC-001
- **Responsible Component:** cp-incident, event-ingest
- **Dependencies:** DLP-POLICY-015
- **API:** gRPC EventIngest; REST /api/v1/incidents
- **Data Model:** incident, event
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Повторная доставка события не создаёт дубль; каждый инцидент соответствует реальному решению; генераторы случайных событий отсутствуют вне тестов.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Правила группировки появляются в DLP-INCIDENT-003.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INCIDENT-002 — Severity Classification

- **Description:** Определять серьёзность по правилу политики с контролируемым ручным изменением.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-INC-002
- **Responsible Component:** cp-incident
- **Dependencies:** DLP-INCIDENT-001
- **API:** REST /api/v1/incidents
- **Data Model:** incident
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Серьёзность воспроизводима по политике; ручное изменение требует причины и попадает в аудит.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Шкала подлежит согласованию с SOC.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INCIDENT-003 — Event Correlation

- **Description:** Связывать события одного пользователя, файла и периода в один случай по детерминированным правилам.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-INC-003
- **Responsible Component:** cp-incident
- **Dependencies:** DLP-INCIDENT-001
- **API:** REST /api/v1/incidents
- **Data Model:** incident, event_link
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Набор сценариев даёт ожидаемые группы; связи объяснимы и обратимы оператором.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Корреляция между каналами зависит от полноты идентификаторов.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INCIDENT-004 — Risk Scoring

- **Description:** Вычислять объяснимую оценку риска по настраиваемым факторам.
- **Priority:** P2 (релиз R1.0)
- **Functional Requirements:** FR-INC-004
- **Responsible Component:** cp-incident
- **Dependencies:** DLP-INCIDENT-003
- **API:** REST /api/v1/incidents
- **Data Model:** risk_score
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Расчёт воспроизводим и показывает вклад каждого фактора; доступ ограничен, соответствие требованиям к персональным данным подтверждено.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Оценка не должна быть единственным основанием кадровых решений; использование требует правовой оценки.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INCIDENT-005 — Investigation Workflow

- **Description:** Вести инцидент по конечному автомату статусов с обязательными причинами переходов.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-INC-005
- **Responsible Component:** cp-incident
- **Dependencies:** DLP-INCIDENT-001
- **API:** REST /api/v1/incidents/{id}/transitions
- **Data Model:** incident, incident_transition
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Недопустимый переход отклоняется; закрытие требует резолюции; вся история сохраняется.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Настраиваемые пользовательские процессы не входят в первую версию.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INCIDENT-006 — Evidence Management

- **Description:** Хранить зашифрованные доказательства с контролем целостности, минимизацией содержимого и журналом доступа.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-INC-006
- **Responsible Component:** cp-incident
- **Dependencies:** DLP-CORE-006, DLP-CORE-002
- **API:** REST /api/v1/incidents/{id}/evidence
- **Data Model:** evidence
- **Implementation Status:** Proposed
- **Acceptance Criteria:** По умолчанию хранятся хеш и маскированный фрагмент; доступ к полному содержимому требует отдельного права и фиксируется; изменение обнаруживается по хешу.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Сохранение полной копии создаёт вторичный риск и должно быть явно разрешено политикой.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INCIDENT-007 — Incident Assignment

- **Description:** Назначать инциденты аналитикам и группам с контролем конфликта интересов.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-INC-007
- **Responsible Component:** cp-incident
- **Dependencies:** DLP-INCIDENT-005
- **API:** REST /api/v1/incidents/{id}/assignment
- **Data Model:** incident
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Назначение возможно только пользователю с правом расследования; инцидент о самом аналитике не назначается ему.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Автоматическое распределение не входит в первую версию.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INCIDENT-008 — Incident Audit Trail

- **Description:** Фиксировать все просмотры, изменения и выгрузки инцидента.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-INC-008
- **Responsible Component:** cp-incident, cp-audit
- **Dependencies:** DLP-CORE-003
- **API:** REST /api/v1/incidents/{id}/history
- **Data Model:** audit_event
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Просмотр доказательства и любое изменение отражаются в истории; историю нельзя изменить через API.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Нет.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INCIDENT-009 — Incident Export

- **Description:** Экспортировать инцидент с манифестом хешей и подписью при наличии права.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-INC-009
- **Responsible Component:** cp-incident
- **Dependencies:** DLP-INCIDENT-006
- **API:** REST /api/v1/incidents/export
- **Data Model:** export_job
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Получатель проверяет целостность пакета по манифесту; экспорт требует отдельного права и попадает в аудит.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Юридическая значимость выгрузки определяется процедурами организации.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INCIDENT-010 — Retention Policies

- **Description:** Удалять инциденты, события и доказательства по срокам с поддержкой удержания на время расследования.
- **Priority:** P0 (релиз R0.4)
- **Functional Requirements:** FR-INC-010
- **Responsible Component:** cp-incident
- **Dependencies:** DLP-INCIDENT-006
- **API:** REST /api/v1/retention
- **Data Model:** retention_policy, legal_hold
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Истёкшие данные удаляются вместе с доказательствами и подтверждаются записью аудита; данные на удержании не удаляются.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Сроки определяются законодательством и политикой организации. Удаление из резервных копий выполняется по их сроку.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INCIDENT-011 — SIEM/SOAR Event Forwarding

- **Description:** Передавать инциденты во внешние системы с гарантированной доставкой и минимизацией содержимого.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-INC-011
- **Responsible Component:** cp-integration
- **Dependencies:** DLP-INTEGRATION-005, DLP-INTEGRATION-006
- **API:** Syslog; Webhook
- **Data Model:** integration_config, delivery_log
- **Implementation Status:** Proposed
- **Acceptance Criteria:** При недоступности приёмника события накапливаются и доставляются без потерь в пределах квоты; чувствительные значения не передаются.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Формат отдельных SIEM подтверждается тестами с реальным продуктом.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

### Management Console

#### DLP-CONSOLE-001 — Dashboard

- **Description:** Показывать сводку инцидентов и состояния агентов по реальным данным API.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-UI-001
- **Responsible Component:** console-ui
- **Dependencies:** DLP-INCIDENT-001, DLP-ENDPOINT-010
- **API:** REST /api/v1/reports/summary
- **Data Model:** Агрегаты
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Значения совпадают с прямым запросом к API; при недоступности источника показывается ошибка, а не нули или статические данные.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Настройка виджетов не входит в первую версию.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CONSOLE-002 — Endpoints

- **Description:** Просматривать и управлять агентами, их группами и состоянием.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-UI-002
- **Responsible Component:** console-ui
- **Dependencies:** DLP-ENDPOINT-013
- **API:** REST /api/v1/agents
- **Data Model:** agent
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Каждое действие вызывает реальный API, неразрешённые действия отклоняются сервером; E2E-тест проходит на реальном агенте.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Показываются только возможности, подтверждённые для ОС агента.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CONSOLE-003 — Users and Groups

- **Description:** Просматривать пользователей и группы каталога и их связи с политиками.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-UI-003
- **Responsible Component:** console-ui
- **Dependencies:** DLP-INTEGRATION-001
- **API:** REST /api/v1/directory
- **Data Model:** directory_user, directory_group
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Данные соответствуют снимку каталога и показывают время синхронизации.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Изменение объектов каталога из консоли не поддерживается.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CONSOLE-004 — Policies

- **Description:** Создавать, проверять, публиковать и откатывать политики.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-UI-004
- **Responsible Component:** console-ui
- **Dependencies:** DLP-POLICY-001, DLP-POLICY-014
- **API:** REST /api/v1/policies
- **Data Model:** policy, policy_version
- **Implementation Status:** Proposed
- **Acceptance Criteria:** E2E-сценарий от создания до блокировки выполняется на реальном шлюзе; показываются различия версий и результат симуляции.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Визуальный конструктор ограничен возможностями схемы.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CONSOLE-005 — Incidents

- **Description:** Искать и просматривать инциденты, с появлением DLP-INCIDENT-005 — расследовать их.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-UI-005
- **Responsible Component:** console-ui
- **Dependencies:** DLP-INCIDENT-001, DLP-INCIDENT-002
- **API:** REST /api/v1/incidents
- **Data Model:** incident
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Фильтрация и пагинация проверяются на большом наборе; маскирование и права соблюдаются сервером.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** В R0.2 экран работает только на просмотр; действия расследования появляются в R0.3. Полнотекстовый поиск по доказательствам по умолчанию отключён.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CONSOLE-006 — Data Classification

- **Description:** Управлять схемами классификации и просматривать результаты.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-UI-006
- **Responsible Component:** console-ui
- **Dependencies:** DLP-DISCOVERY-003
- **API:** REST /api/v1/classification
- **Data Model:** classification_label
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Все данные экрана получены от API; изменения версионируются.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Зависит от DLP-DISCOVERY-003.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CONSOLE-007 — Data Discovery

- **Description:** Настраивать цели и расписания и просматривать ход и результаты сканирования.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-UI-007
- **Responsible Component:** console-ui
- **Dependencies:** DLP-DISCOVERY-001
- **API:** REST /api/v1/discovery
- **Data Model:** scan_job, scan_result
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Прогресс отражает реальное задание; учётные данные целей не показываются.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Зависит от DLP-DISCOVERY-001.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CONSOLE-008 — Reports

- **Description:** Формировать и выгружать отчёты по инцидентам, политикам и обнаруженным данным.
- **Priority:** P1 (релиз R0.6)
- **Functional Requirements:** FR-UI-008
- **Responsible Component:** console-ui
- **Dependencies:** DLP-DISCOVERY-006
- **API:** REST /api/v1/reports
- **Data Model:** report
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Итоги сверяются с данными тестом; выгрузка попадает в аудит.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Конструктор произвольных отчётов не входит в первую версию.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CONSOLE-009 — Integrations

- **Description:** Настраивать интеграции и проверять соединение с ними.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-UI-009
- **Responsible Component:** console-ui
- **Dependencies:** DLP-INTEGRATION-005
- **API:** REST /api/v1/integrations
- **Data Model:** integration_config
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Проверка соединения выполняет реальное обращение; секреты после сохранения не отображаются.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Показываются только реализованные интеграции.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CONSOLE-010 — Administration

- **Description:** Управлять операторами, ролями и настройками системы.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-UI-010
- **Responsible Component:** console-ui
- **Dependencies:** DLP-CORE-002, DLP-CORE-007
- **API:** REST /api/v1/rbac; REST /api/v1/system/config
- **Data Model:** role, system_config
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Опасные изменения требуют повторной MFA-проверки; все действия отражаются в аудите.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Набор настроек, доступных из интерфейса, ограничен.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CONSOLE-011 — Audit Logs

- **Description:** Просматривать, фильтровать и выгружать журнал аудита и результат проверки целостности.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-UI-011
- **Responsible Component:** console-ui
- **Dependencies:** DLP-CORE-003
- **API:** REST /api/v1/audit
- **Data Model:** audit_event
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Просмотр доступен только соответствующим ролям; результат верификатора показывается для выбранного диапазона.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Только чтение.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-CONSOLE-012 — System Health

- **Description:** Показывать реальное состояние компонентов, очередей и хранилищ.
- **Priority:** P0 (релиз R0.1)
- **Functional Requirements:** FR-UI-012
- **Responsible Component:** console-ui
- **Dependencies:** DLP-CORE-008
- **API:** REST /api/v1/system/health
- **Data Model:** метрики вне БД
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Остановка компонента отражается на экране в течение согласованного времени; нет статических индикаторов.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Не заменяет внешний мониторинг.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

### Enterprise Integrations

#### DLP-INTEGRATION-001 — Active Directory

- **Description:** Синхронизировать пользователей и группы AD по защищённому соединению и с минимальными правами.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-INT-001
- **Responsible Component:** cp-integration
- **Dependencies:** DLP-CORE-005
- **API:** REST /api/v1/directory; LDAPS
- **Data Model:** directory_user, directory_group, directory_snapshot
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Вложенные группы разрешаются корректно на тестовом домене; недоступность каталога не удаляет данные; используется только LDAPS или StartTLS.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Многодоменные леса подтверждаются отдельно. Вход через Kerberos не входит в R0.3.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INTEGRATION-002 — LDAP

- **Description:** Синхронизировать каталог с LDAP-серверами с настраиваемым отображением атрибутов.
- **Priority:** P0 (релиз R0.3)
- **Functional Requirements:** FR-INT-002
- **Responsible Component:** cp-integration
- **Dependencies:** DLP-CORE-005
- **API:** REST /api/v1/directory; LDAPS
- **Data Model:** directory_user, directory_group
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Совместимость подтверждена с утверждёнными серверами; инъекция в LDAP-фильтры предотвращена и проверена тестами.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Отличия схем требуют настройки отображения.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INTEGRATION-003 — SAML 2.0

- **Description:** Обеспечить вход операторов через SAML 2.0 с проверкой подписи и защитой от повторов.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-INT-003
- **Responsible Component:** cp-identity
- **Dependencies:** DLP-CORE-001
- **API:** SAML 2.0
- **Data Model:** identity_provider
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Тесты на обход подписи и подмену утверждений проходят; совместимость подтверждена с утверждёнными IdP.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** MFA обеспечивается IdP; подтверждение требуется политикой доверия.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INTEGRATION-004 — OpenID Connect

- **Description:** Обеспечить вход операторов через OIDC с Authorization Code Flow и PKCE.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-INT-004
- **Responsible Component:** cp-identity
- **Dependencies:** DLP-CORE-001
- **API:** OIDC
- **Data Model:** identity_provider
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Проверка издателя, аудитории, nonce и PKCE покрыта негативными тестами; совместимость подтверждена с утверждёнными IdP.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Режим без доступа к IdP в контуре невозможен.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INTEGRATION-005 — Syslog

- **Description:** Передавать аудит и события по Syslog через TLS с очередью и повторами.
- **Priority:** P0 (релиз R0.4)
- **Functional Requirements:** FR-INT-005
- **Responsible Component:** cp-integration
- **Dependencies:** DLP-CORE-003
- **API:** Syslog (RFC 5424, TLS)
- **Data Model:** integration_config, delivery_log
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Сообщения соответствуют RFC 5424 в контрактном тесте; при обрыве доставка возобновляется; потери учитываются.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Гарантия доставки ограничена размером очереди.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INTEGRATION-006 — SIEM

- **Description:** Предоставить документированные форматы событий для SIEM с сопоставлением ключевых полей.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-INT-006
- **Responsible Component:** cp-integration
- **Dependencies:** DLP-INTEGRATION-005
- **API:** Syslog; REST /api/v1/events
- **Data Model:** integration_config
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Схема событий версионирована; приём проверен на утверждённой SIEM.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Перечень продуктов не утверждён (OQ-07).
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INTEGRATION-007 — SOAR

- **Description:** Позволить SOAR получать и обновлять инциденты через API и Webhooks.
- **Priority:** P2 (релиз R1.0)
- **Functional Requirements:** FR-INT-007
- **Responsible Component:** cp-integration
- **Dependencies:** DLP-INTEGRATION-009, DLP-INTEGRATION-008
- **API:** Webhook; REST /api/v1/incidents
- **Data Model:** integration_config
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Плейбук утверждённого продукта проходит полный цикл; права учётной записи интеграции минимальны.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Готовые коннекторы к конкретным SOAR не предполагаются до определения спроса.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INTEGRATION-008 — Public REST API

- **Description:** Предоставить внешним системам API с сервисными учётными записями и ограниченными правами.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-INT-008
- **Responsible Component:** cp-platform
- **Dependencies:** DLP-CORE-012
- **API:** REST /api/v1
- **Data Model:** api_client, api_token
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Токены имеют срок и область, хранятся только как хеши и отзываются немедленно; запросы попадают в аудит согласно политике.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Набор публичных операций расширяется по мере готовности функций.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INTEGRATION-009 — Webhooks

- **Description:** Отправлять подписанные события с повторами и защитой от SSRF.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-INT-009
- **Responsible Component:** cp-integration
- **Dependencies:** DLP-CORE-005
- **API:** Webhook (HTTPS)
- **Data Model:** webhook_subscription, delivery_log
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Получатель проверяет HMAC-подпись и метку времени; повторы идемпотентны; адреса внутренних сервисов запрещены по умолчанию.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Порядок доставки не гарантируется.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INTEGRATION-010 — SMTP Notifications

- **Description:** Отправлять уведомления операторам и пользователям через корпоративный SMTP без чувствительного содержимого.
- **Priority:** P1 (релиз R0.5)
- **Functional Requirements:** FR-INT-010
- **Responsible Component:** cp-integration
- **Dependencies:** DLP-CORE-005
- **API:** SMTP (TLS)
- **Data Model:** notification_template, delivery_log
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Шаблоны исключают открытые значения; ошибка доставки регистрируется и повторяется; инъекция заголовков предотвращена.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Канал email не является гарантированным для критических оповещений.
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INTEGRATION-011 — Proxy Compatibility Profiles

- **Description:** Вести подтверждённые тестами профили совместимости с корпоративными прокси.
- **Priority:** P0 (релиз R0.2)
- **Functional Requirements:** FR-INT-011
- **Responsible Component:** gw-icap
- **Dependencies:** DLP-NETWORK-003
- **API:** ICAP
- **Data Model:** Нет
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Каждый профиль содержит версию продукта, настройки и автоматизированный тест; неподтверждённые продукты не заявляются.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Перечень зависит от инфраструктуры заказчика (OQ-07).
- **Change History:** CHG-0001 (2026-11-09) — запись создана

#### DLP-INTEGRATION-012 — Fortinet Security Fabric

- **Description:** Исследовать и при наличии официально поддерживаемых интерфейсов реализовать интеграцию с Fortinet Security Fabric.
- **Priority:** P3 (релиз Post-1.0)
- **Functional Requirements:** FR-INT-012
- **Responsible Component:** cp-integration
- **Dependencies:** DLP-INTEGRATION-005, DLP-NETWORK-004
- **API:** Определяется после исследования
- **Data Model:** integration_config
- **Implementation Status:** Proposed
- **Acceptance Criteria:** Выбранный интерфейс подтверждён документацией производителя и тестом на реальном оборудовании или официальном образе.
- **Test References:** Нет — тесты не созданы
- **Known Limitations:** Статус возможности не подтверждён; функция не заявляется до исследования, требуется лицензирование и доступ к стенду.
- **Change History:** CHG-0001 (2026-11-09) — запись создана
