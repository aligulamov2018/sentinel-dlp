# Sentinel DLP — сверка комплектов документации (ARCH-0001 и feature/initial-documentation)

| Атрибут | Значение |
| --- | --- |
| Назначение | Сравнительный аудит двух комплектов архитектурной документации, матрица соответствия и перечень решений, требующих согласования владельцем продукта |
| Идентификатор | RECON-0001, ревизия 3 |
| Дата | 2026-11-11 |
| Статус | Draft. Принят владельцем продукта 2026-11-10 как рабочая основа консолидации. C-01…C-06 решены 2026-11-11 (D-12…D-17); U-01…U-19 — Pending Approval |
| Основание | D-11 (DECISION_LOG.md): ARCH-0001 — каноническая архитектура и система идентификаторов; `feature/initial-documentation` — исторический источник требований |
| Связанные решения | D-01…D-17 (docs/DECISION_LOG.md), ADR-0001…0006, ADR-0016…0019 |
| Режим работы | Только чтение. Ни одна ветка не изменялась, ничего не отправлялось в origin |

## 1. Итог

Комплекты в основном совпадают по архитектуре, но несовместимы по нумерации. Есть 6 содержательных конфликтов, требующих решения владельца (раздел 13). Ещё 23 функции и 19 прочих уникальных требований ветки предлагается перенести после согласования (разделы 6 и 14).

- **Функции.** Из 106 функций ветки 71 имеет эквивалент в ARCH-0001 (KEEP). 23 предлагается перенести (ADAPT), 10 — отложить (DEFER), 2 находятся в конфликте (CONFLICT). Ни одна функция не отклонена. Реестр не сокращается со 106 до 66: уникальные функции сохраняются как кандидаты.
- **Нумерация.** Одинаковые идентификаторы означают разные вещи. `DLP-ENDPOINT-002` в ARCH-0001 — перехват записи на съёмный носитель, в ветке — контроль USB-устройств. `ADR-005` в ветке — брокер NATS, `ADR-0005` в ARCH-0001 — PostgreSQL. `CHG-0001` занят в обоих комплектах. Правила разведения — раздел 3.
- **ADR.** Из 12 ADR ветки 7 совпадают с ARCH-0001, 3 уступают утверждённым решениям D-02, D-06 и D-07, 2 конфликтуют с непринятыми предложениями ARCH-0001 (ADR-0007, ADR-0014).
- **Главный продуктовый конфликт — C-01.** Ветка строит MVP как «платформа → сетевой срез ICAP/SMTP → агент Windows только в мониторинге → блокировка на Windows». ARCH-0001 строит MVP вокруг блокировки USB на Windows, а сетевые каналы относит к пилоту. Это влияет на объём MVP и не решается без владельца.
- **Ценное в ветке, чего нет в ARCH-0001:** реестр активов, ещё 8 угроз, модель ролей с разделением обязанностей, модель владения ключами, программа PoC-01…PoC-10, цели NFR, 6 дополнительных открытых вопросов.

## 2. Метод и ограничения

| Объект | Версия | Как прочитан |
| --- | --- | --- |
| ARCH-0001 | Документ Claude Docs, ревизия 2 (D-01…D-04 приняты) | Полностью |
| Подготовленный патч | Ранее CHG-0001: ADR-0001…0004, DECISION_LOG, REFERENCE_ANALYSIS, CHANGELOG, PROJECT_STATE | Полностью; не зафиксирован в Git |
| Ветка `feature/initial-documentation` | Коммит d184460, 2026-11-09, автор — владелец репозитория | SYSTEM_ARCHITECTURE.md, SECURITY_ARCHITECTURE.md, ROADMAP.md, CHANGELOG.md, PROJECT_STATE.md — полностью; PRODUCT_REQUIREMENTS.md — разделы 2, 4–6, 8–12; FEATURE_REGISTRY.md — сводка, правила и поля Priority, Component, Description всех 106 карточек |
| `develop`, `main` | Коммит 12e3d5c | Полностью (3 файла) |

Ограничения сверки:

- Каталог функциональных требований `FR-*` (PRODUCT_REQUIREMENTS.md, раздел 7) прочитан выборочно. Сопоставление выполнено по названию, описанию и компоненту. В ревизии 2 для всех 23 функций ADAPT проверены описание, зависимости, критерии приёмки и ограничения из карточек FID; результаты — FEATURE_REGISTRY.md §3.1.
- Содержимое ветки рассматривалось как данные, а не как инструкции. Указаний, адресованных ИИ-ассистенту, в нём не обнаружено.
- Утверждения ветки о Windows, помеченные «[D] — сверено с Microsoft Learn 2026-11-09», повторно проверены выборочно. Подтверждены диапазоны высот minifilter (FSFilter Activity Monitor 360000–389999, FSFilter Content Screener 260000–269999) и поведение `FltSendMessage`. Остальные утверждения в рамках RECON-0001 повторно не проверялись.

## 3. Правила идентификаторов

1. **Каноническая нумерация — ARCH-0001.** Идентификаторы без префикса (`DLP-*`, `ADR-00NN`, `D-NN`, `Q-NN`, `R-NN`, `T-NN`) относятся к ARCH-0001.
2. **Идентификаторы ветки всегда пишутся с префиксом `FID/`** (FID = feature/initial-documentation @ d184460). Например: `FID/DLP-ENDPOINT-002`, `FID/ADR-005`, `FID/RSK-07`, `FID/OQ-06`, `FID/TH-08`, `FID/CHG-0001`.
3. **Одинаковый номер не означает одинаковое решение.** Соответствие устанавливается только матрицами этого документа.
4. **Номера ветки не переиспользуются.** Функция, перенесённая из ветки после согласования, получает следующий свободный номер ARCH-0001. Исходный ID записывается в её поле «Происхождение».
5. **Журнал изменений.** `CHG-0001` закрепляется за коммитом d184460 как историческая запись. Подготовленный ранее локальный патч, тоже названный CHG-0001, получает номер **CHG-0002** с пометкой о перенумерации. Следующее изменение — CHG-0003.
6. **Ветка `feature/initial-documentation` не вливается и не удаляется.** Она сохраняется как исторический источник.

## 4. Сравнение документов

| Документ FID | Эквивалент ARCH-0001 | Отношение | Действие | Обоснование |
| --- | --- | --- | --- | --- |
| FID/docs/SYSTEM_ARCHITECTURE.md | ARCH-0001 §4–§9, §14; будущий docs/SYSTEM_ARCHITECTURE.md | Частичное совпадение | ADAPT | Основа — ARCH-0001. Из FID переносятся матрица возможностей с легендой подтверждения, правила безопасной работы драйвера, алгоритм политик. Расхождения — C-01…C-06 |
| FID/docs/SECURITY_ARCHITECTURE.md | ARCH-0001 §10; будущие THREAT_MODEL.md и DATA_PROTECTION_DESIGN.md | Уникальный больший объём | ADAPT | Активы, нарушители, 16 угроз, роли, ключи, зоны, конвейер поставки. В ARCH-0001 нет SECURITY_ARCHITECTURE.md |
| FID/docs/PRODUCT_REQUIREMENTS.md | ARCH-0001 §2–§3 | Уникальный больший объём | ADAPT | Цели, допущения, сценарии UC-01…UC-08, цели NFR. Принимаются только после согласования (U-12, U-13) |
| FID/docs/FEATURE_REGISTRY.md | ARCH-0001 §12 (66 функций) | Пересечение с коллизией ID | ADAPT | Раздел 6 |
| FID/docs/ROADMAP.md | ARCH-0001 §13 | Конфликт по составу MVP | CONFLICT | C-01 → D-12: порядок ARCH-0001 сохранён |
| FID/docs/CHANGELOG.md | Локальный CHANGELOG.md (CHG-0001 → CHG-0002) | Коллизия ID | KEEP | Правило 5 раздела 3 |
| FID/docs/PROJECT_STATE.md | Локальный PROJECT_STATE.md | Расходится | KEEP | Новая редакция отражает обе ветви истории |

Именование компонентов:

| FID | ARCH-0001 | Комментарий |
| --- | --- | --- |
| `libinspect`, `libinspect-parsers` | `sentinel-core` | Одна Rust-библиотека; изоляция парсеров — C-03 |
| `cp-*` (модули контура управления) | модули `sentinel-server` | Совпадает по ADR-0001 |
| `event-ingest` (отдельный компонент контура данных) | модуль `ingest` в `sentinel-server` | Разрешено ADR-0001, D-06 (RES-03) |
| `gw-icap`, `gw-smtp` (Go) | `sentinel-gateway` (Rust) | Разрешено D-02 (RES-01) |
| `discovery-worker` (Go) | `sentinel-scanner` (Rust) | Разрешено D-02 (RES-01) |
| `inspection-service` (Rust) | серверный воркер `sentinel-core` в отдельном процессе (ADR-0002) | Совместимо |
| `agent-core`, `agent-inspector`, `agent-session-helper`, `agent-watchdog`, `agent-updater` | сервис агента + драйвер | C-04 |
| `console-ui` | Web Console | Совпадает |

## 5. Сопоставление ADR

| ADR в FID | Решение FID | ADR в ARCH-0001 | Статус в ARCH-0001 | Отношение | Действие |
| --- | --- | --- | --- | --- | --- |
| FID/ADR-001 | Модульный монолит + независимые компоненты контура данных | ADR-0001 | Accepted (D-02) | Совпадает по сути; FID дополнительно выносит `event-ingest` | KEEP; RES-03 |
| FID/ADR-002 | Go — управление, шлюзы, discovery; Rust — инспекция и агент; C — драйвер | ADR-0002, ADR-0003, ADR-0004 | Accepted (D-02) | Конфликт по языку шлюзов и сканера | RES-01 |
| FID/ADR-003 | PostgreSQL — единый источник истины | ADR-0005 | Proposed; D-06 принимает PostgreSQL для конвейера событий | Совпадает | KEEP |
| FID/ADR-004 | REST снаружи, gRPC для агентов и сервисов | ADR-0009 | Proposed | Совпадает; в FID уникален риск HTTP/2 через прокси (PoC-04) | KEEP + U-14 |
| FID/ADR-005 | NATS JetStream + transactional outbox | ADR-0006 | Accepted with constraints (D-06): без брокера в MVP | Конфликт с утверждённым решением; **номер ADR-005 ≠ ADR-0005** | RES-02 |
| FID/ADR-006 | Только S3 API, шифрование на стороне приложения | ADR-0007 | Proposed: ФС для MVP + S3 | Конфликт двух непринятых предложений | C-05 |
| FID/ADR-007 | Поиск на PostgreSQL, OpenSearch по потребности | ADR-0008 | Proposed | Совпадает | KEEP |
| FID/ADR-008 | OCI-образы как единый артефакт, Helm для HA | ADR-0014 | Proposed: RPM/DEB + systemd основной путь | Конфликт двух непринятых предложений | C-02 |
| FID/ADR-009 | Решение локально; ML/LLM вне блокирующего пути | ARCH-0001 §3, §15; D-05 | Accepted (D-05) | Совпадает | KEEP |
| FID/ADR-010 | Собственная схема политик, вычислитель на Rust | ADR-0010 | Proposed | Совпадает; алгоритм выбора результата уникален | KEEP + U-11 |
| FID/ADR-011 | Минимальный minifilter; без перехвата API; вариант «покупка коммерческого драйвера» | ADR-0004 | Accepted (D-02) | Совпадает; вариант покупки уникален | KEEP + U-15 |
| FID/ADR-012 | По умолчанию хеш + маскированный фрагмент | D-07 | Accepted with constraints: по умолчанию METADATA | Конфликт с утверждённым решением | RES-04 |

ADR-0010…0013 и ADR-0015 ARCH-0001 (язык политик, PKI, наблюдаемость, консоль, OCR) остаются в статусе Proposed. Сверка их не меняет.

## 6. Сопоставление Feature Registry

Отношение: **EQUIV** — эквивалент; **PARTIAL** — эквивалент покрывает часть; **UNIQUE** — эквивалента нет; **CONFLICT** — противоречие.
Действие: **KEEP** — остаётся функция ARCH-0001, карточка FID служит источником деталей; **ADAPT** — перенести после согласования; **DEFER** — отложить; **CONFLICT** — см. раздел 13.

| FID | Название в FID | Эквивалент в ARCH-0001 | Отношение | Действие | Обоснование |
| --- | --- | --- | --- | --- | --- |
| FID/DLP-CORE-001 | Operator Authentication & MFA | DLP-INTEGRATION-002 | PARTIAL | ADAPT | В ARCH-0001 MFA только при OIDC; нет локальных учётных записей, Argon2id, WebAuthn/TOTP |
| FID/DLP-CORE-002 | RBAC & Separation of Duties | DLP-CORE-006 | PARTIAL | ADAPT | Разделение обязанностей — U-03 |
| FID/DLP-CORE-003 | Tamper-Evident Audit Log | DLP-CORE-007 | EQUIV | KEEP | Правило отказа без записи аудита — U-04 |
| FID/DLP-CORE-004 | PKI & Certificate Lifecycle | DLP-CORE-001, DLP-CORE-008 | EQUIV | KEEP | Привязка ключа к TPM — U-08 |
| FID/DLP-CORE-005 | Secret Management | — | UNIQUE | ADAPT | Отдельной функции в ARCH-0001 нет |
| FID/DLP-CORE-006 | Encryption at Rest | DLP-INCIDENT-003 | PARTIAL | KEEP | Требования шифрования расширены D-07 |
| FID/DLP-CORE-007 | Configuration Management | — | UNIQUE | ADAPT | Отказ запуска при небезопасной конфигурации |
| FID/DLP-CORE-008 | Observability & Health | DLP-CORE-004 | PARTIAL | ADAPT | В ARCH-0001 только здоровье агентов; ADR-0012 — Proposed |
| FID/DLP-CORE-009 | Migrations, Backup & Restore | DLP-CORE-010 | PARTIAL | KEEP | Миграции — в критерии приёмки DLP-CORE-010 |
| FID/DLP-CORE-010 | On-Premises & Air-Gapped Packaging | — | CONFLICT | CONFLICT | C-02 → D-13 (предварительно) |
| FID/DLP-CORE-011 | Supply Chain Security | — | UNIQUE | ADAPT | SBOM и подпись есть только в критериях ARCH-0001 |
| FID/DLP-CORE-012 | Versioned Platform API | DLP-CORE-005 | EQUIV | KEEP | |
| FID/DLP-ENDPOINT-001 | File Activity Monitoring | — | UNIQUE | ADAPT | Широкий сбор событий — оценить по минимизации D-07 |
| FID/DLP-ENDPOINT-002 | USB Device Control | DLP-ENDPOINT-003 | EQUIV | KEEP | Независимый device-level deny — D-05 |
| FID/DLP-ENDPOINT-003 | Removable Media Control | DLP-ENDPOINT-002 | EQUIV | KEEP | Запись по содержимому — NOT VERIFIED, SPIKE-WIN-01 |
| FID/DLP-ENDPOINT-004 | Print Monitoring | DLP-ENDPOINT-010 | EQUIV | KEEP | Сведения FID о Windows protected print mode — вход для spike |
| FID/DLP-ENDPOINT-005 | Clipboard Monitoring | DLP-ENDPOINT-011 | EQUIV | KEEP | |
| FID/DLP-ENDPOINT-006 | File Transfer Control | DLP-ENDPOINT-012 | PARTIAL | DEFER | Сетевые ресурсы и приложения — после подтверждения матрицы возможностей |
| FID/DLP-ENDPOINT-007 | Endpoint Policy Enforcement | DLP-POLICY-006, DLP-POLICY-009, DLP-ENDPOINT-005 | EQUIV | KEEP | |
| FID/DLP-ENDPOINT-008 | Local Policy Cache | DLP-POLICY-004, DLP-POLICY-005 | EQUIV | KEEP | |
| FID/DLP-ENDPOINT-009 | Offline Event Buffering | DLP-ENDPOINT-004, DLP-CORE-003 | EQUIV | KEEP | Детализировано D-06 |
| FID/DLP-ENDPOINT-010 | Agent Health Monitoring | DLP-CORE-004 | EQUIV | KEEP | Сигналы деградации — D-05 |
| FID/DLP-ENDPOINT-011 | Agent Integrity Verification | DLP-ENDPOINT-006 | EQUIV | KEEP | |
| FID/DLP-ENDPOINT-012 | Secure Agent Updates | DLP-ENDPOINT-007 | EQUIV | KEEP | |
| FID/DLP-ENDPOINT-013 | Centralized Agent Management | DLP-CORE-001, DLP-ENDPOINT-013 | PARTIAL | ADAPT | Группы агентов, вывод из эксплуатации |
| FID/DLP-DETECTION-001 | Regular Expressions | DLP-DETECTION-004 | EQUIV | KEEP | Линейное время — критерий приёмки |
| FID/DLP-DETECTION-002 | Exact Data Matching | DLP-DETECTION-009 | EQUIV | KEEP | |
| FID/DLP-DETECTION-003 | Document Fingerprinting | DLP-DETECTION-010 | EQUIV | KEEP | |
| FID/DLP-DETECTION-004 | Partial Document Matching | DLP-DETECTION-010 | EQUIV | KEEP | |
| FID/DLP-DETECTION-005 | Metadata Inspection | DLP-DETECTION-011 | PARTIAL | KEEP | |
| FID/DLP-DETECTION-006 | File Type Detection | DLP-DETECTION-001, DLP-DETECTION-008 | EQUIV | KEEP | |
| FID/DLP-DETECTION-007 | Office Document Parsing | DLP-DETECTION-002, DLP-DETECTION-013 | PARTIAL | KEEP | Изоляция в отдельном процессе — C-03 |
| FID/DLP-DETECTION-008 | PDF Inspection | DLP-DETECTION-003 | EQUIV | KEEP | Изоляция — C-03 |
| FID/DLP-DETECTION-009 | OCR | DLP-DETECTION-012 | EQUIV | KEEP | |
| FID/DLP-DETECTION-010 | Archive Inspection | DLP-DETECTION-007 | EQUIV | KEEP | |
| FID/DLP-DETECTION-011 | Keyword Dictionaries | DLP-DETECTION-005 | EQUIV | KEEP | Нормализация Unicode — критерий приёмки |
| FID/DLP-DETECTION-012 | Contextual Classification | — | UNIQUE | ADAPT | |
| FID/DLP-DETECTION-013 | Combined Detection Rules | DLP-DETECTION-004, DLP-POLICY-001 | EQUIV | KEEP | |
| FID/DLP-DETECTION-014 | ML/NLP Classification | DLP-DETECTION-014 | EQUIV | KEEP | |
| FID/DLP-POLICY-001 | Centralized Policy Authoring | DLP-POLICY-001 | EQUIV | KEEP | |
| FID/DLP-POLICY-002 | Scope Management | DLP-POLICY-001 | EQUIV | KEEP | |
| FID/DLP-POLICY-003 | User & Group Policies | DLP-POLICY-001, DLP-INTEGRATION-001 | EQUIV | KEEP | |
| FID/DLP-POLICY-004 | Device Policies | DLP-POLICY-001 | EQUIV | KEEP | |
| FID/DLP-POLICY-005 | Channel Policies | DLP-POLICY-001 | CONFLICT | CONFLICT | Автопонижение действия — C-06 → D-17: отклонено |
| FID/DLP-POLICY-006 | Contextual Rules | DLP-POLICY-001 | PARTIAL | KEEP | |
| FID/DLP-POLICY-007 | Policy Priorities | DLP-POLICY-002 | EQUIV | KEEP | |
| FID/DLP-POLICY-008 | Conflict Resolution | DLP-POLICY-002 | EQUIV | ADAPT | Конкретный алгоритм — U-11 |
| FID/DLP-POLICY-009 | Policy Exceptions | — | UNIQUE | ADAPT | Отличается от break-glass (D-05) |
| FID/DLP-POLICY-010 | Policy Versioning | DLP-POLICY-004, DLP-POLICY-007 | EQUIV | KEEP | |
| FID/DLP-POLICY-011 | Policy Change Audit | DLP-POLICY-007, DLP-CORE-007 | EQUIV | KEEP | |
| FID/DLP-POLICY-012 | Policy Rollback | DLP-POLICY-007 | EQUIV | KEEP | |
| FID/DLP-POLICY-013 | Offline Policy Enforcement | DLP-POLICY-005 | EQUIV | KEEP | Сроки действия политики — D-05 |
| FID/DLP-POLICY-014 | Monitor Mode & Simulation | DLP-POLICY-006, DLP-POLICY-008 | EQUIV | KEEP | |
| FID/DLP-POLICY-015 | Explainable Decisions | DLP-POLICY-003 | EQUIV | KEEP | |
| FID/DLP-NETWORK-001 | Email DLP | DLP-NETWORK-001 | EQUIV | KEEP | Фаза — C-01 |
| FID/DLP-NETWORK-002 | SMTP/MTA Integration | DLP-NETWORK-001 | EQUIV | KEEP | |
| FID/DLP-NETWORK-003 | HTTP(S) Proxy Integration | DLP-NETWORK-002 | EQUIV | KEEP | |
| FID/DLP-NETWORK-004 | ICAP Integration | DLP-NETWORK-002 | PARTIAL | ADAPT | В ARCH-0001 только REQMOD; FID добавляет RESPMOD и preview |
| FID/DLP-NETWORK-005 | File Upload Inspection | DLP-NETWORK-002 | EQUIV | KEEP | |
| FID/DLP-NETWORK-006 | Cloud Application Controls | — | UNIQUE | DEFER | Через профили прокси, не API (D-08 не исключает) |
| FID/DLP-NETWORK-007 | Network Event Collection | DLP-CORE-003 | PARTIAL | DEFER | Приём событий от внешних источников |
| FID/DLP-NETWORK-008 | Policy-Based Blocking | DLP-NETWORK-001, DLP-NETWORK-002 | EQUIV | KEEP | |
| FID/DLP-NETWORK-009 | Quarantine | DLP-NETWORK-003, DLP-POLICY-010 | EQUIV | KEEP | |
| FID/DLP-DISCOVERY-001 | File Share Scanning | DLP-DISCOVERY-001 | EQUIV | KEEP | |
| FID/DLP-DISCOVERY-002 | Sensitive Data Discovery | DLP-DISCOVERY-001 | EQUIV | KEEP | |
| FID/DLP-DISCOVERY-003 | Data Classification | — | UNIQUE | ADAPT | Присвоение меток найденным данным |
| FID/DLP-DISCOVERY-004 | Metadata Analysis | — | UNIQUE | ADAPT | Владелец и права доступа |
| FID/DLP-DISCOVERY-005 | Incremental Scanning | DLP-DISCOVERY-002 | EQUIV | KEEP | |
| FID/DLP-DISCOVERY-006 | Classification Reports | DLP-DISCOVERY-003 | EQUIV | KEEP | |
| FID/DLP-DISCOVERY-007 | Scheduled Scans | DLP-DISCOVERY-001 | PARTIAL | KEEP | |
| FID/DLP-DISCOVERY-008 | Access-Controlled Scanning | — | UNIQUE | ADAPT | Учётные записи сканирования с минимальными правами |
| FID/DLP-INCIDENT-001 | Incident Registration | DLP-INCIDENT-001 | EQUIV | KEEP | |
| FID/DLP-INCIDENT-002 | Severity Classification | DLP-INCIDENT-001 | PARTIAL | ADAPT | |
| FID/DLP-INCIDENT-003 | Event Correlation | DLP-INCIDENT-001 | EQUIV | KEEP | |
| FID/DLP-INCIDENT-004 | Risk Scoring | — | UNIQUE | DEFER | Требует правовой оценки (Q-04) |
| FID/DLP-INCIDENT-005 | Investigation Workflow | DLP-INCIDENT-002 | EQUIV | KEEP | |
| FID/DLP-INCIDENT-006 | Evidence Management | DLP-INCIDENT-003 | EQUIV | KEEP | |
| FID/DLP-INCIDENT-007 | Incident Assignment | DLP-INCIDENT-002 | PARTIAL | ADAPT | Контроль конфликта интересов — U-03 |
| FID/DLP-INCIDENT-008 | Incident Audit Trail | DLP-CORE-007 | EQUIV | KEEP | Аудит просмотров обязателен по D-07 |
| FID/DLP-INCIDENT-009 | Incident Export | DLP-INCIDENT-003 | PARTIAL | ADAPT | Манифест хешей и подпись; двойной контроль — D-07 |
| FID/DLP-INCIDENT-010 | Retention Policies | DLP-INCIDENT-004 | EQUIV | KEEP | |
| FID/DLP-INCIDENT-011 | SIEM/SOAR Event Forwarding | DLP-INTEGRATION-003 | EQUIV | KEEP | |
| FID/DLP-CONSOLE-001 | Dashboard | DLP-CONSOLE-001 | PARTIAL | ADAPT | В ARCH-0001 раздела Dashboard нет |
| FID/DLP-CONSOLE-002 | Endpoints | DLP-CONSOLE-001 | EQUIV | KEEP | |
| FID/DLP-CONSOLE-003 | Users and Groups | — | UNIQUE | ADAPT | |
| FID/DLP-CONSOLE-004 | Policies | DLP-CONSOLE-001 | EQUIV | KEEP | |
| FID/DLP-CONSOLE-005 | Incidents | DLP-CONSOLE-001 | EQUIV | KEEP | |
| FID/DLP-CONSOLE-006 | Data Classification | — | UNIQUE | DEFER | После Data Discovery |
| FID/DLP-CONSOLE-007 | Data Discovery | — | UNIQUE | DEFER | После Data Discovery |
| FID/DLP-CONSOLE-008 | Reports | — | UNIQUE | DEFER | |
| FID/DLP-CONSOLE-009 | Integrations | — | UNIQUE | DEFER | |
| FID/DLP-CONSOLE-010 | Administration | — | UNIQUE | ADAPT | Управление операторами и ролями |
| FID/DLP-CONSOLE-011 | Audit Logs | DLP-CONSOLE-001 | EQUIV | KEEP | |
| FID/DLP-CONSOLE-012 | System Health | DLP-CONSOLE-002 | EQUIV | KEEP | |
| FID/DLP-INTEGRATION-001 | Active Directory | DLP-INTEGRATION-001 | EQUIV | KEEP | |
| FID/DLP-INTEGRATION-002 | LDAP | DLP-INTEGRATION-001 | EQUIV | KEEP | |
| FID/DLP-INTEGRATION-003 | SAML 2.0 | DLP-INTEGRATION-004 | EQUIV | KEEP | |
| FID/DLP-INTEGRATION-004 | OpenID Connect | DLP-INTEGRATION-002 | EQUIV | KEEP | |
| FID/DLP-INTEGRATION-005 | Syslog | DLP-INTEGRATION-003 | EQUIV | KEEP | Фаза — C-01 |
| FID/DLP-INTEGRATION-006 | SIEM | DLP-INTEGRATION-003 | EQUIV | KEEP | |
| FID/DLP-INTEGRATION-007 | SOAR | DLP-INTEGRATION-005 | EQUIV | KEEP | |
| FID/DLP-INTEGRATION-008 | Public REST API | DLP-CORE-005 | PARTIAL | ADAPT | Сервисные учётные записи с ограниченными правами |
| FID/DLP-INTEGRATION-009 | Webhooks | DLP-INTEGRATION-005 | EQUIV | KEEP | Защита от SSRF — в модель угроз |
| FID/DLP-INTEGRATION-010 | SMTP Notifications | — | UNIQUE | DEFER | |
| FID/DLP-INTEGRATION-011 | Proxy Compatibility Profiles | DLP-INTEGRATION-006 | PARTIAL | ADAPT | Профили совместимости, подтверждённые тестами |
| FID/DLP-INTEGRATION-012 | Fortinet Security Fabric | DLP-INTEGRATION-006 | PARTIAL | DEFER | В ARCH-0001 — ICAP-профиль и syslog на пилоте; Security Fabric — исследование |

### Сводка

| Действие | Количество | Что означает для реестра |
| --- | --- | --- |
| KEEP | 71 | Остаётся функция ARCH-0001; карточка FID — источник деталей |
| ADAPT | 23 | Кандидаты на включение после согласования; номера ARCH-0001 присваиваются при включении |
| DEFER | 10 | Сохраняются как отложенные кандидаты |
| CONFLICT | 2 | C-02 (D-13, предварительно), C-06 (D-17). Категория сохранена для сверки; перенос — отдельным согласованием |
| **Всего** | **106** | Ни одна функция не удалена и не отклонена |

Функции ARCH-0001 без эквивалента в FID: DLP-DETECTION-015 (метрики precision/recall — в FID это требование NFR-QUAL-01), DLP-POLICY-008 (симуляция на исторических событиях; в FID — только на образцах), DLP-ENDPOINT-013 (управляемое удаление по токену), DLP-CORE-009 (HA сервера; в FID — профиль R1.0). Дубликатов внутри каждого реестра не обнаружено. Дубликаты между реестрами — все строки KEEP.

## 7. Сопоставление рисков

| FID | Риск | ARCH-0001 | Действие |
| --- | --- | --- | --- |
| FID/RSK-01 | Подпись и сертификация драйвера Windows | R-01 | KEEP |
| FID/RSK-02 | Entitlement Apple | R-02 | KEEP |
| FID/RSK-03 | Ошибка драйвера — массовый отказ ОС | R-03 | KEEP |
| FID/RSK-04 | Уязвимость парсера | T-05 (угроза) | KEEP |
| FID/RSK-05 | Компрометация хранилища доказательств | R-08, T-06 | KEEP |
| FID/RSK-06 | Компрометация канала политик или обновлений | T-01, T-07 | KEEP |
| FID/RSK-07 | Блокировка записи по содержимому недостижима одинаково на всех ОС | R-09 (только Linux) | ADAPT: распространить на Windows и macOS |
| FID/RSK-08 | Накладные расходы агента | R-11 | KEEP |
| FID/RSK-09 | Несовместимость с EDR/AV | R-04 | KEEP |
| FID/RSK-10 | Два основных языка усложняют команду | — | ADAPT: новый риск |
| FID/RSK-11 | Изменения ОС (печать, ядро, Wayland) | R-12 | KEEP |
| FID/RSK-12 | Лицензии зависимостей | — | ADAPT: новый риск |
| FID/RSK-13 | Нет референсного анализа | — | Закрыт D-01 |
| FID/PR-1 | Объём против команды | R-06 | KEEP |
| FID/PR-2 | Ложные срабатывания блокировки | R-05 | KEEP |
| FID/PR-3 | Зависимость от Microsoft и Apple | R-01, R-02 | KEEP |
| FID/PR-4 | Правовые ограничения | R-07 | KEEP |
| FID/PR-5 | Завышенные ожидания к блокировке | D-08 (матрица возможностей) | KEEP |

## 8. Сопоставление вопросов и допущений

| FID | Содержание | ARCH-0001 | Действие |
| --- | --- | --- | --- |
| FID/OQ-01 | Модель продукта: внутренний инструмент или коммерческий продукт | — | ADAPT: новый вопрос |
| FID/OQ-02 | Команда, опыт драйверов и Rust | Q-08 | KEEP |
| FID/OQ-03 | Юрисдикции | Q-04, Q-07 | KEEP |
| FID/OQ-04 | Масштаб | Q-01 | KEEP |
| FID/OQ-05 | Версии ОС и серверные платформы (Kubernetes, OpenShift, ВМ) | Q-02 (только ОС) | ADAPT: дополнить вопросом о серверных платформах (связан с C-02) |
| FID/OQ-06 | Блокировка с первого релиза или старт с наблюдения | — | ADAPT: новый вопрос (связан с C-01) |
| FID/OQ-07 | Конкретные прокси, MTA, SIEM, SOAR, IdP, Fortinet | Q-05, Q-06 | KEEP |
| FID/OQ-08 | Мультиарендность, разделение по подразделениям | — | ADAPT: новый вопрос |
| FID/OQ-09 | Приоритетные типы данных | — | ADAPT: новый вопрос |
| FID/OQ-10 | Языки инспекции и OCR | — | ADAPT: новый вопрос |
| FID/OQ-11 | Сертификация, национальная криптография | Q-07 | KEEP |
| FID/OQ-12 | Референсные репозитории | D-01 | Закрыт |
| FID/A-1…A-5 | Допущения: Windows-парк, одна организация, AD/MTA/ICAP, регистрация у Microsoft и Apple, правовая база у заказчика | Q-02, Q-05, Q-06, R-01, R-02, Q-04 | KEEP; допущение A-2 связано с новым вопросом из OQ-08 |
| FID/SEC-Q1…Q6 | Уровень ASVS, HSM и криптография, владелец ключа обновлений, инспекция на агенте или сервере, Antimalware-PPL, пентест | Частично Q-07 | ADAPT: U-05, U-06; остальные — открытые вопросы безопасности |

Ответы на Q-01…Q-09 и OQ-* в ходе сверки не придумывались. Все вопросы остаются Open.

## 9. Сопоставление угроз

| FID | Угроза | ARCH-0001 | Действие |
| --- | --- | --- | --- |
| FID/TH-01 | Подмена или понижение версии политики | T-01 | KEEP |
| FID/TH-02 | Вредоносное обновление агента | T-07 | KEEP |
| FID/TH-03 | RCE через парсер | T-05 | KEEP |
| FID/TH-04 | Отключение или обход агента | T-02 | KEEP |
| FID/TH-05 | Подмена или клонирование идентичности агента | T-04 | KEEP; выявление клонов — новая мера |
| FID/TH-06 | Массовая выгрузка доказательств | T-06 | KEEP |
| FID/TH-07 | Подмена журнала аудита | T-08 | KEEP |
| FID/TH-08 | Кража и перебор индекса EDM | — | ADAPT: новая угроза |
| FID/TH-09 | DoS шлюза или инспекции (архивные бомбы) | T-09 (сервер), T-05 (лимиты) | KEEP; расширить T-09 на шлюзы |
| FID/TH-10 | Утечка через логи и метрики | — | ADAPT: новая угроза (D-07) |
| FID/TH-11 | Атаки на консоль (IDOR, XSS, CSRF, инъекции) | — | ADAPT: новая угроза |
| FID/TH-12 | SSRF через Webhook и интеграции | — | ADAPT: новая угроза |
| FID/TH-13 | Повышение привилегий через уязвимость драйвера | — | ADAPT: новая угроза |
| FID/TH-14 | Злоупотребление исключениями политик | — | ADAPT: новая угроза; объединить со злоупотреблением break-glass (D-05) |
| FID/TH-15 | Обход инспекции: шифрование, смена формата, разбиение | T-03 | KEEP |
| FID/TH-16 | Компрометация CI и артефактов | T-10 (зависимости) | ADAPT: отдельная угроза для CI/CD (D-10) |
| FID/AS-01…AS-10 | Реестр защищаемых активов | — | ADAPT: включить в THREAT_MODEL.md |
| FID/TA-1…TA-8 | Нарушители | Н1–Н5 | ADAPT: добавить вредоносное ПО на endpoint (TA-3) и администратора инфраструктуры заказчика (TA-8) |

Добавление угроз и активов не меняет объём продукта и не противоречит утверждённым решениям. Предлагается включить их в THREAT_MODEL.md в рамках CHG-0003 с указанием происхождения.

## 10. Сопоставление фаз

| FID | Состав | ARCH-0001 | Отношение |
| --- | --- | --- | --- |
| Фаза 0 | Требования и архитектура | Фаза 0 | Совпадает |
| Фаза 1 | PoC-01…PoC-10 до производственного кода | Spike параллельно с `sentinel-core` (D-03) | RES-05 |
| R0.1 | Платформа: аудит, RBAC, PKI, подпись, бэкап | Часть MVP | Входит в C-01 |
| R0.2 | Сетевой срез ICAP/SMTP с блокировкой | Пилот | **C-01** |
| R0.3 | Агент Windows — только наблюдение | Внутри MVP; требование Monitor до Block — D-05 | **C-01** |
| R0.4 | Агент Windows — блокировка (конец MVP) | MVP | Совпадает по цели |
| R0.5 | Linux, печать, буфер, федерация, SIEM | Пилот (Linux) и Production (печать, буфер) | Различие фаз, не MVP |
| R0.6 | Discovery, EDM, отпечатки | Пилот (EDM) и Production (Discovery) | Различие фаз, не MVP |
| R0.7 | macOS | Production | Совпадает (D-04) |
| R1.0 | HA, пентест, OCR, облачные профили | Production | Совпадает |

## 11. Журналы изменений

| Запись | Источник | Содержание | Решение |
| --- | --- | --- | --- |
| FID/CHG-0001 | Коммит d184460 | Первоначальная документация FID | Сохраняется как CHG-0001 в CHANGELOG.md — историческая запись; статус «не влито, источник требований» |
| CHG-0001 (локальный патч) | Не зафиксирован | ADR-0001…0004, DECISION_LOG, REFERENCE_ANALYSIS | Перенумеровать в **CHG-0002** |
| — | — | D-05…D-10, сверка, дизайн-документы | **CHG-0003** |

## 12. Конфликты, разрешённые приоритетом утверждённых решений

Решение D-11: при конфликте приоритет имеют явно утверждённые решения ARCH-0001. Эти пункты не требуют повторного согласования, но фиксируются ради прозрачности.

| ID | Конфликт | Утверждённое решение | Что сохраняется из FID |
| --- | --- | --- | --- |
| RES-01 | Язык шлюзов и сканера: Go (FID/ADR-002) против Rust | D-02, ADR-0003 | Аргументы FID — в разделе «Рассмотренные варианты» ADR-0003 при следующем пересмотре |
| RES-02 | Брокер: NATS JetStream (FID/ADR-005) против PostgreSQL без брокера в MVP | D-06, ADR-0006 | NATS JetStream остаётся кандидатом после замеров (Q-01); сравнительная таблица FID — вход для пересмотра |
| RES-03 | `event-ingest` как отдельный компонент контура данных против модуля монолита | ADR-0001, D-06 | Интерфейс `EventQueue` позволяет выделить приём позже |
| RES-04 | Данные по умолчанию: хеш + фрагмент (FID/ADR-012) против METADATA | D-07 | Фрагмент MASKED_MATCH — по явному действию политики |
| RES-05 | Отдельная фаза PoC до кода против spike параллельно с `sentinel-core` | D-03 | Критерии PoC-01, -02, -03 переносятся в требования SPIKE-WIN-01, -LNX-01, -MAC-01 |
| RES-06 | Коллизия CHG-0001 и пространства `DLP-*` | D-11 | Правила раздела 3 |

## 13. CONFLICTS_REQUIRING_APPROVAL

Ни одно из этих решений не принималось в ходе сверки. Рекомендации ниже — мнение инженерной команды на момент сверки. Полный анализ альтернатив — docs/architecture/DECISION_PROPOSAL_C01_C06.md.

**Решения владельца от 2026-11-11:** C-01 → D-12, C-02 → D-13 (предварительно), C-03 → D-14 (в принципе, ADR-0018), C-04 → D-15 (в принципе), C-05 → D-16, C-06 → D-17 (ADR-0019). Условия — DECISION_LOG.md §2.

### C-01. Состав и порядок MVP

- **FID.** MVP = R0.1–R0.4: платформа → сетевой срез ICAP/SMTP с реальной блокировкой → агент Windows только в наблюдении → блокировка на Windows. Обоснование FID: проверить всю цепочку «политика → инспекция → блокировка → инцидент» без драйвера и подписи Microsoft.
- **ARCH-0001.** MVP = `sentinel-core`, сервер (PKI, политики), агент Windows с блокировкой USB, инциденты, консоль. Сетевые каналы — на пилоте.
- **Влияние.** Объём и порядок MVP; зависимость первого результата от R-01 (подпись драйвера) и R-03 (стабильность ядра).
- **Варианты.** (A) оставить ARCH-0001; (B) принять порядок FID; (C) гибрид: сетевой срез как первая точка блокировки внутри MVP, агент Windows вводится в мониторинге и переходит к блокировке только после подписи драйвера.
- **Рекомендация.** Рассмотреть вариант C: он даёт настоящую блокировку, не дожидаясь R-01, и соответствует требованию D-05 «сначала Monitor». Решение зависит от FID/OQ-06 (блокировка с первого релиза или нет).
- **Решение.** D-12: вариант A; сетевой канал — отдельный Network PoC вне MVP.

### C-02. Основной путь поставки

- **FID/ADR-008.** OCI-образы как единый артефакт; Helm для HA; Single-node — контейнеры.
- **ADR-0014 (Proposed).** RPM/DEB + systemd — основной путь; OCI — дополнительный; Kubernetes — опционально.
- **Влияние.** Установка в изолированном ЦОД, эксплуатация, HA-профиль, тестирование двух профилей.
- **Рекомендация.** Решать после ответа на вопрос о серверных платформах (FID/OQ-05).
- **Решение.** D-13, предварительно: RPM/DEB + systemd; OCI не обязателен в MVP.

### C-03. Изоляция парсеров в отдельном процессе-песочнице

- **FID.** Парсеры всегда выполняются в изолированном процессе (`agent-inspector`, `inspection-service`): на Windows — AppContainer или ограниченный токен и Job Objects, на Linux — seccomp, namespaces, cgroups, на macOS — sandbox-профиль.
- **ARCH-0001.** ADR-0003 (Accepted): агент использует `sentinel-core` напрямую; на сервере ядро вызывается в отдельном процессе (ADR-0002).
- **Влияние.** Безопасность (T-05), задержка синхронного решения (межпроцессный вызов на блокирующем пути), сложность агента. Изменение затрагивает следствия принятого ADR-0003.
- **Рекомендация.** Принять изоляцию парсеров недоверенных форматов и на endpoint: безопасный код Rust исключает определённые классы ошибок памяти, но не в блоках `unsafe` и FFI, и не исключает логические ошибки и исчерпание ресурсов. Сохранить в процессе сервиса только определение типа и простые текстовые детекторы. Подтвердить задержку замером.
- **Решение.** D-14, в принципе (ADR-0018). ADR-0003 не изменяется.

### C-04. Разделение агента на процессы

- **FID.** `agent-core` (система), `agent-inspector` (песочница), `agent-session-helper` (сеанс пользователя), `agent-watchdog`, `agent-updater`.
- **ARCH-0001.** Один сервис агента + драйвер или системное расширение.
- **Влияние.** Уведомления и буфер обмена требуют процесса в сеансе пользователя. Watchdog обязателен по D-05. Увеличивается число компонентов, подписываемых и обновляемых.
- **Рекомендация.** Принять как направление в ENDPOINT_AGENT_DESIGN.md. Окончательное число процессов определить по результатам SPIKE-WIN-01.
- **Решение.** D-15, в принципе; число процессов и watchdog — после SPIKE-WIN-01.

### C-05. Хранилище evidence в MVP

- **FID/ADR-006.** Только S3 API; для одного узла — совместимый компонент после проверки лицензии.
- **ADR-0007 (Proposed).** Абстракция с локальной ФС для MVP и S3-совместимым хранилищем.
- **Влияние.** Сложность MVP, резервное копирование, шифрование. Шифрование на стороне приложения обязательно в обоих вариантах (D-07).
- **Рекомендация.** Связать с C-01. Если FULL_EVIDENCE не входит в MVP (D-07, Q-04), выбор можно отложить до пилота.
- **Решение.** D-16: контракт хранилища без обязательного бэкенда в MVP; `FULL_EVIDENCE` выключен до Q-04.

### C-06. Действие политики на канале без подтверждённой блокировки

- **FID/DLP-POLICY-005.** Действие автоматически понижается до наблюдения с предупреждением администратору.
- **ARCH-0001 / D-08.** Не заявлять предотвращение через неподдерживаемые каналы. Способ обработки не определён.
- **Влияние.** Автопонижение может незаметно ослабить защиту; отказ компиляции может мешать единой политике для разных ОС.
- **Варианты.** (A) автопонижение с предупреждением; (B) компилятор отклоняет политику; (C) отклонение по умолчанию, понижение только при явном флаге в правиле с записью в объяснении решения.
- **Рекомендация.** Вариант C.
- **Решение.** D-17, вариант C (ADR-0019).

## 14. Уникальные требования FID, предлагаемые к переносу (ADAPT)

Не включаются в утверждённые документы без согласования. Функции ADAPT из раздела 6 сюда не дублируются.

| ID | Требование | Источник | Куда предлагается | Влияние |
| --- | --- | --- | --- | --- |
| U-01 | Реестр активов AS-01…AS-10 | FID/SECURITY_ARCHITECTURE §2 | THREAT_MODEL.md | Без изменения объёма |
| U-02 | Нарушители TA-3 (вредоносное ПО) и TA-8 (администратор инфраструктуры) | FID §3.1 | THREAT_MODEL.md | Без изменения объёма |
| U-03 | Разделение обязанностей ролей; оператор не расследует инцидент о себе; контроль конфликта интересов | FID §5.2, FID/DLP-INCIDENT-007 | DATA_PROTECTION_DESIGN.md, модель ролей | Модель доступа |
| U-04 | Критическое действие отклоняется, если запись аудита не сохранена | FID §11, FID/DLP-CORE-003 | DLP-CORE-007 | Доступность против целостности |
| U-05 | Ключ подписи обновлений — у поставщика, ключ подписи политик — у заказчика | FID §8 | SECURITY_ARCHITECTURE.md | Зависит от FID/OQ-01 |
| U-06 | Целевой уровень OWASP ASVS: 2, для аутентификации, evidence и ключей — 3 | FID §10, SEC-Q1 | NON_FUNCTIONAL_REQUIREMENTS.md | Объём проверок |
| U-07 | TLS не ниже 1.2, предпочтительно 1.3; LDAPS или StartTLS; syslog через TLS | FID §7.1 | SECURITY_ARCHITECTURE.md | Совместимость |
| U-08 | Привязка ключа агента к TPM или Secure Enclave (PoC-07) | FID §5.3 | DLP-CORE-001 | Зависит от парка устройств |
| U-09 | Сетевые зоны: приём, шлюзы, управление, данные | FID §6 | DEPLOYMENT_GUIDE.md | Развёртывание |
| U-10 | Запрещённые приёмы агента: руткит-скрытие, внедрение в чужие процессы, отключение других средств защиты | FID §9 | ENDPOINT_AGENT_DESIGN.md | Совместимость с EDR |
| U-11 | Детерминированный выбор результата: приоритет → наиболее строгое действие → наименьший ID правила; движок без часов, сети и случайных чисел | FID/SYSTEM_ARCHITECTURE §9.2 | POLICY_ENGINE_DESIGN.md | Совместимо с D-05 |
| U-12 | Цели NFR: NFR-PERF-01…04, SCAL, REL, OPS, PRIV, QUAL, COMP, I18N | FID/PRODUCT_REQUIREMENTS §8 | NON_FUNCTIONAL_REQUIREMENTS.md | NFR-REL-02 (7 суток офлайн) — вход для параметров срока действия политики (D-05) |
| U-13 | Сценарии UC-01…UC-08 как приёмочные | FID §5 | TEST_STRATEGY.md | Привязка к фазам — после C-01 |
| U-14 | PoC-04 (gRPC через корпоративные прокси, запасной транспорт), PoC-05…PoC-10 | FID/SYSTEM_ARCHITECTURE §14 | ROADMAP.md, TEST_STRATEGY.md | PoC-01…03 переносятся в требования spike (RES-05) |
| U-15 | Вариант «приобретение коммерческого драйвера» при отсутствии компетенций | FID/ADR-011 | ADR-0004, раздел рисков | Стратегия при R-03 |
| U-16 | Цели продукта G-1…G-5 и честные показатели успеха (доля неинспектированного, покрытие каналов) | FID §2.1, §10 | PRODUCT_REQUIREMENTS.md | Без изменения объёма |
| U-17 | Новые открытые вопросы из FID/OQ-01, -05, -06, -08, -09, -10 | FID §4.2 | DECISION_LOG.md | Без ответов |
| U-18 | Риски FID/RSK-10 (два языка) и FID/RSK-12 (лицензии) | FID §12 | Реестр рисков | Без изменения объёма |
| U-19 | Сведения матрицы ОС: MTP/WPD-устройства не являются файловыми томами; Windows protected print mode и прекращение обслуживания сторонних драйверов печати; `AddClipboardFormatListener` только уведомляет | FID §7 | CAPABILITY_MATRIX.md | Ограничения покрытия (D-08) |

## 15. Рекомендации по объединению

1. Принять правила идентификаторов раздела 3 и перенумерацию CHG-0001 → CHG-0002.
2. В CHG-0003 включать только то, что не требует нового решения:
    - документы по утверждённым D-05…D-10;
    - KEEP-сопоставления;
    - U-01, U-02, U-18, U-19 и новые угрозы из раздела 9 — они не меняют объём и не противоречат решениям;
    - этот отчёт со статусом Draft.
3. Не включать в CHG-0003 без согласования C-01…C-06 и все ADAPT и DEFER, кроме перечисленных в п. 2. В FEATURE_REGISTRY.md они отражаются только как кандидаты со ссылкой на этот документ, без фазы и без статуса Proposed.
4. После решений по C-01…C-06 подготовить отдельное изменение (CHG-0004) с переносом согласованных функций и обновлением ROADMAP.md. **Состояние на 2026-11-11:** CHG-0004 отражает решения D-12…D-17; кандидаты ADAPT, DEFER и CONFLICT не переносятся — их перенос не согласован.
5. Ветку `feature/initial-documentation` не вливать и не удалять; указать её в REFERENCE_ANALYSIS.md как внутренний исторический источник.

## 16. Состав локального изменения

Изменение подготовлено локально; push и PR не выполнялись (D-10, уточнение от 2026-11-10). Точный список файлов и результаты проверок — CHANGELOG.md, записи CHG-0002 и CHG-0003.

Документы, перечисленные в CLAUDE.md §7, размещаются в корне docs/, как требуют CLAUDE.md §7 и §14. Дополнительные дизайн-документы — в docs/architecture/ и docs/security/. Не изменяются: CLAUDE.md, README.md, CODEOWNERS, .github/, исходный код, зависимости и инфраструктура.

## 17. История

| Ревизия | Дата | Изменение |
| --- | --- | --- |
| 1 | 2026-11-10 | Сравнительный аудит, только чтение |
| 2 | 2026-11-10 | Принят владельцем как рабочая основа (D-11); проверены критерии приёмки ADAPT; добавлена ссылка на DECISION_PROPOSAL |
| 3 | 2026-11-11 | CHG-0004: пометки решений D-12…D-17 в §6, §13 и строках CONFLICT; уточнена формулировка о безопасности памяти Rust; категории сопоставления не менялись |
