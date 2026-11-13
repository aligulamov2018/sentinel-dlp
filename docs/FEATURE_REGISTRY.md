# Sentinel DLP — Feature Registry

| Атрибут | Значение |
| --- | --- |
| Назначение | Единый каталог функций Sentinel DLP (CLAUDE.md §8) |
| Версия | 7 (CHG-0007 rev3.2: ограничение DLP-DETECTION-005 для PDP v0.1 по решению CHG7-OD2; 6 — CHG-0007: DLP-POLICY-001, -002, -003, -011 — In Progress по TR-0001; 5 — CHG-0004: решения D-12…D-17; ранее — ревью AR-01…AR-08, SR2-01…SR2-04, SR3-02) |
| Дата | 2026-11-13 |
| Статус документа | Proposed — на архитектурном ревью |
| Основание | ARCH-0001 §12; D-03, D-05…D-08, D-12…D-17; RECON-0001 |
| Связанные документы | docs/ROADMAP.md, docs/SYSTEM_ARCHITECTURE.md, docs/architecture/DOCUMENTATION_RECONCILIATION.md, docs/TEST_STRATEGY.md |

**Фактическое состояние: реализованных функций нет. Все функции имеют статус Proposed.**

## Правила ведения

- Статусы жизненного цикла: Proposed, Designed, In Progress, Implemented, Tested, Integrated, Production Ready, Deprecated (CLAUDE.md §8). Переход требует доказательства: ссылки на дизайн, код и результаты тестов.
- **Основание** — отдельное поле, показывающее, кто определил объём функции:
  - `D-NN` — объём функции следует из утверждённого решения;
  - `ARCH-0001` — проектное предложение, не утверждённое владельцем.
- Кандидаты из ветки `feature/initial-documentation` ведутся отдельно (раздел 3), под исходными идентификаторами с префиксом `FID/`. Окончательные ID ARCH-0001 им не присваиваются до согласования.
- Фаза — предложение ROADMAP.md (MVP, Пилот, Production). Не является утверждённым графиком, кроме решений D-03 и D-04.
- Идентификаторы не переиспользуются. Ненужная функция переводится в Deprecated.
- Полная карточка (описание, требования, API, модель данных, критерии приёмки, тесты, ограничения, история) заполняется при переходе в Designed. Для функций раздела 1 краткие критерии приёмки приведены уже сейчас.

## 1. Функции, объём которых определён утверждёнными решениями

| ID | Функция | Компонент | Фаза | Статус | Основание | Критерий приёмки (кратко) | Тесты |
| --- | --- | --- | --- | --- | --- | --- | --- |
| DLP-DETECTION-001 | Определение реального типа файла по сигнатурам | Detection Engine | MVP | Proposed | D-03 | Тип определяется независимо от расширения на эталонном корпусе | Unit, корпус |
| DLP-DETECTION-002 | Извлечение текста: TXT, CSV, OOXML | Detection Engine | MVP | Proposed | D-03 | Текст извлекается в лимитах ресурсов; превышение лимита → NOT_INSPECTED | Unit, TC-SEC-04 |
| DLP-DETECTION-003 | Извлечение текста из PDF | Detection Engine | MVP | Proposed | D-03 | То же | Unit, TC-SEC-04 |
| DLP-DETECTION-004 | Regex-детекторы с валидаторами и порогом количества | Detection Engine | MVP | Proposed | D-03 | Линейное время по длине входа; точность измерена | Unit, DLP-DETECTION-015 |
| DLP-DETECTION-005 | Словари ключевых слов с весами | Detection Engine | MVP | Proposed | D-03 | Нормализация регистра и Unicode, включая русский язык | Unit |
| DLP-DETECTION-006 | Маскирование совпадений до выхода из процесса инспекции | Detection Engine | MVP | Proposed | D-03, D-07 | Открытые значения не покидают инспекцию; правила DATA_PROTECTION_DESIGN.md §3 | TC-D07-07, TC-D07-09 |
| DLP-DETECTION-015 | Метрики precision и recall на эталонном корпусе | Detection Engine | MVP | Proposed | D-03 | Метрики по каждому детектору записаны в TEST_RESULTS.md | Корпус |
| DLP-POLICY-001 | Схема политик: область, условия, действия, исключения | Policy Engine | MVP | In Progress (CHG-0007: IR и структурная проверка SENTINEL_CORE_DESIGN.md §6.7 без подписи, манифеста и формата бандла; O-05, O-06 открыты) | D-03 | Схема валидируется; неизвестные атрибуты обрабатываются явным правилом | Unit, Contract; TR-0001 |
| DLP-POLICY-002 | Детерминированная оценка и разрешение конфликтов по приоритету | Policy Engine | MVP | In Progress (CHG-0007: алгоритм SENTINEL_CORE_DESIGN.md §6.3 реализован; G-01…G-30 и оракул PASS; побайтный детерминизм на двух ОС — NOT RUN, ИТ-13) | D-03 | Golden-тесты: одинаковый вход → одинаковое решение | Golden; TR-0001 |
| DLP-POLICY-003 | Объяснение решения | Policy Engine | MVP | In Progress (CHG-0007: `Explanation` SENTINEL_CORE_DESIGN.md §6.6 реализовано; сериализация — нет) | D-03 | Объяснение содержит версию, правила, в том числе проигравшие, детекторы, причину | Golden; TR-0001 |
| DLP-POLICY-004 | Подписанный бандл политики, защита от отката | Policy Engine | MVP | Proposed | D-03, D-05 | Неподписанный или устаревший бандл отвергается | TC-D05-05 |
| DLP-POLICY-005 | Офлайн-исполнение по кэшированной политике | Policy Engine | MVP | Proposed | D-03, D-05 | Решения не зависят от доступности сервера | TC-D06-01, TC-D05-06 |
| DLP-POLICY-006 | Режимы Monitor, Alert, Block | Policy Engine | MVP | Proposed | D-03, D-05 | Блокирующее правило публикуется только после периода Monitor | Unit, Integration |
| DLP-POLICY-011 | Контракт вердикта ALLOW / BLOCK / INDETERMINATE с причинами | Policy Engine | MVP | In Progress (CHG-0007: типы §5 и вердикт PDP; интеграция с хостом и PEP — нет) | D-05 | INDETERMINATE всегда с причиной из FAILURE_MODE_DESIGN.md §2.3 | Unit, TC-D05-08; TR-0001 |
| DLP-POLICY-012 | Fallback `on_indeterminate` по каналу и операции с ограничениями компиляции | Policy Engine | MVP | Proposed | D-05, D-17 | BLOCK допускается только для защищаемых операций на подтверждённых платформах; без подтверждённого BLOCK_CAPABLE — ошибка компиляции даже при исключении понижения основного BLOCK (ADR-0019) | TC-D08-02, TC-D17-03 |
| DLP-POLICY-013 | Срок действия политики: `soft_ttl`, `hard_expiry_at`, сохраняемое между перезагрузками время, защита от отката часов; поведение после истечения — Proposed | Policy Engine | MVP | Proposed | D-05 | Откат часов, восстановление старого `time_state` и перезагрузки не продлевают разрешающие отступления; нет глобальной блокировки и глобального fail-open (INV-T1…T5) | TC-D05-06, TC-D05-13…16, TC-D05-19…21 |
| DLP-POLICY-014 | Уровень передачи данных в действии правила (METADATA, MASKED_MATCH, FULL_EVIDENCE) и уровень пути (`path_level`) | Policy Engine | MVP (без FULL_EVIDENCE) | Proposed | D-07, D-16 | По умолчанию METADATA и псевдоним пути; MASKED_MATCH — только по явному действию правила; полный путь — только по явному уровню FULL; FULL_EVIDENCE выключен на уровне платформы и не включается до Q-04 | TC-D07-09, TC-D07-10, TC-D16-01 |
| DLP-POLICY-015 | Проверка возможностей платформы при компиляции и исполнении; запрет тихого понижения; контролируемые исключения понижения | Policy Engine | MVP | Proposed | D-08, D-05, D-17 | Инварианты CAPABILITY_MATRIX.md §4; BLOCK без подтверждённого BLOCK_CAPABLE — `CAPABILITY_INSUFFICIENT`; понижение — только по исключению с областью «платформа, канал, операция», обоснованием и утверждением (ADR-0019) | TC-D08-01…08, TC-D17-01…06 |
| DLP-ENDPOINT-004 | Локальная очередь событий: SQLite WAL с шифрованием, классы и квоты | Endpoint Agent | MVP | Proposed | D-06 | P1 не вытесняются тихо; маркеры `EVENTS_LOST` в слоте первого неотправленного события; события `IN_FLIGHT` не вытесняются; режим `TELEMETRY_INCOMPLETE` | TC-D06-06, TC-D06-40 |
| DLP-ENDPOINT-014 | Break-glass: область, срок, проверка полномочий, аудит, двойное подтверждение; в офлайне действует только в текущем запуске сервиса | Endpoint Agent, Management Server | MVP | Proposed | D-05 | Истекает без участия оператора; повтор кода отклоняется | TC-SEC-11, TC-D05-17, TC-D05-22 |
| DLP-ENDPOINT-015 | Watchdog и восстановление сервиса агента | Endpoint Agent | MVP | Proposed | D-05, D-15 | Цикл сбоев ведёт в `DEGRADED`, а не в бесконечный перезапуск; механизм (восстановление средствами службы ОС или отдельный watchdog) выбирается после SPIKE-WIN-01 | TC-D05-07, TC-D15-01 |
| DLP-ENDPOINT-016 | Поведение до старта сервиса и bootstrap-политика | Endpoint Agent | MVP | Proposed | D-05 | Поведение соответствует FAILURE_MODE_DESIGN.md §6.2–§6.3; гарантия fail-closed не заявляется без подтверждения | TC-D05-10 |
| DLP-ENDPOINT-017 | Исследование: device-level deny недоверенных USB-устройств | Endpoint Agent | Фаза 0 (исследование) | Proposed | D-05 | Отчёт по трём ОС; не заменяет контентную DLP | TC-SEC-10 |
| DLP-ENDPOINT-018 | Сигналы деградации защиты; состояния PEP и PDP в heartbeat; обнаружение потери контроля канала; фиксация неисполненных действий (`ENFORCEMENT_FAILED`) | Endpoint Agent, Management Server | MVP | Proposed | D-05, D-08 | Каждый переход состояния — событие P1; `CONTROL_LOST` не позже `T_detect`; `UNKNOWN` при отсутствии heartbeat | TC-D05-01…07, TC-D08-03, TC-D08-06…08 |
| DLP-ENDPOINT-019 | Контентная блокировка записи на съёмный носитель с гарантией неизменности проверенных байтов (L3) | Endpoint Agent | После SPIKE-WIN-01 | Proposed | D-05 | Записанные байты совпадают с проанализированными, иначе блокировка или INDETERMINATE(`CONTENT_CHANGED`); до SPIKE-WIN-01 не заявляется (R-15) | TC-D05-09, TC-D05-18 |
| DLP-ENDPOINT-020 | Исследование: защита состояния времени от отката на основе NV-счётчика TPM | Endpoint Agent | Фаза 0 (исследование) | Proposed | D-05 | Отчёт по доступности, ресурсу записи, vTPM, Linux, macOS; не считается подтверждённым механизмом до отдельного решения | TC-D05-20 |
| DLP-ENDPOINT-021 | Изоляция разбора недоверенных форматов на агенте: рабочий процесс с ограниченными правами, лимиты ресурсов средствами ОС, IPC с версиями | Endpoint Agent | MVP (только форматы MVP) | Proposed | D-14 | Рискованные форматы не разбираются в привилегированном процессе; сбой, зависание и исчерпание ресурсов рабочего процесса дают `NOT_INSPECTED(reason)`, дальше — вердикт PDP и fallback по ADR-0016; разбор использует тот же `sentinel-core`; механизм NOT VERIFIED (ADR-0018) | TC-D14-01…06, TC-SEC-04 |
| DLP-CORE-003 | Приём событий с глобальной дедупликацией и ACK после фиксации | Management Server | MVP | Proposed | D-06 | Ровно одна запись на ключ при конкурентных, перекрывающихся и переупорядоченных пакетах; закрытие пропуска не блокируется квотой и окном; декларации потерь неизменяемы; ACK только после `COMMIT` | TC-D06-01…04, TC-D06-12…17, TC-D06-22…29, TC-D06-31 |
| DLP-CORE-007 | Неизменяемый журнал аудита с цепочкой хешей; аудит просмотра, экспорта и удаления | Management Server | MVP | Proposed | D-07 | Изменение записи обнаруживается | TC-D07-08 |
| DLP-CORE-011 | Transactional outbox и воркеры: атомарный захват, аренда с fencing-токеном, восстановление, повторы, dead letter | Management Server | MVP | Proposed | D-06 | Задание не выдаётся двум воркерам; устаревший воркер не может подтвердить задание; сбой не теряет задание | TC-D06-05, TC-D06-11, TC-D06-18…21, TC-D06-30 |
| DLP-CORE-012 | Backpressure, ограничение частоты, приоритет событий безопасности | Management Server | MVP | Proposed | D-06 | Перегрузка → `RESOURCE_EXHAUSTED` с `retry_after`; P1 в приоритете | TC-D06-07 |
| DLP-CORE-013 | Время событий, расхождение часов, защита от повторов и конфликтов целостности | Management Server | MVP | Proposed | D-06 | `clock_suspect`; секционирование по `received_at`; карантин конфликтов | TC-D06-09, TC-D06-10 |
| DLP-CORE-014 | Конвертное шифрование evidence; возможность интеграции с KMS/HSM | Management Server | Пилот | Proposed | D-07 | DEK на объект; KEK вне БД и вне её резервных копий | TC-D07-03, TC-D07-04 |
| DLP-CORE-015 | Внешнее закрепление целостности аудита | Management Server | Пилот | Proposed | D-07 | Контрольные точки во внешней системе; расхождение обнаруживается | TC-D07-08 |
| DLP-INCIDENT-003 | Хранилище evidence с шифрованием и контролем целостности | Incident Management | Пилот | Proposed | D-07, D-16 | Изменение объекта обнаруживается при чтении; бэкенд выбирается к пилоту и проходит контрактные тесты хранилища | TC-D07-03, TC-D16-02 |
| DLP-INCIDENT-004 | Сроки хранения и legal hold | Incident Management | Пилот | Proposed | D-07 | Удаление по сроку; при legal hold — нет | TC-D07-05 |
| DLP-INCIDENT-005 | Права доступа к инцидентам и evidence с областью, обоснованием и повторной MFA | Incident Management | MVP | Proposed | D-07 | Доступ вне области отклоняется; каждый просмотр аудируется | TC-D07-01, TC-D07-06 |
| DLP-INCIDENT-006 | Двойной контроль массового экспорта и досрочного удаления | Incident Management | Пилот | Proposed | D-07 | Без второго лица операция невозможна; самоподтверждение запрещено | TC-D07-02 |
| DLP-INCIDENT-007 | Удаление evidence через crypto-shredding: ключи периода, уничтожение KEK, журнал удалений при восстановлении | Incident Management | Пилот | Proposed | D-07 | Удалённое evidence не расшифровывается после уничтожения KEK периода и не возвращается при восстановлении копии | TC-D07-05, TC-D07-11, TC-D07-12 |
| DLP-PLATFORM-001 | Безопасное журналирование: список разрешённых полей, ограниченные дампы | Все компоненты | MVP | Proposed | D-07 | Тест с маркером проходит | TC-D07-07 |
| DLP-PLATFORM-002 | Защита резервных копий: шифрование, раздельные ключи, двойной контроль восстановления | Management Server | Пилот | Proposed | D-07 | Копия без KEK не раскрывает evidence | TC-D07-04 |

## 2. Проектные предложения ARCH-0001 (не утверждены)

| ID | Функция | Компонент | Приоритет | Фаза | Платформы | Статус | Примечание |
| --- | --- | --- | --- | --- | --- | --- | --- |
| DLP-CORE-001 | Регистрация агента по одноразовому токену, выпуск сертификата | Management Server | P0 | MVP | — | Proposed | `agent_epoch` при регистрации (D-06) |
| DLP-CORE-002 | mTLS-канал агент ↔ сервер (gRPC) | Management Server | P0 | MVP | — | Proposed | ADR-0009 Proposed |
| DLP-CORE-004 | Heartbeat и статус здоровья агентов | Management Server | P0 | MVP | — | Proposed | Включает `EffectiveCapabilities` (CAPABILITY_MATRIX.md §4.2) |
| DLP-CORE-005 | REST API с OpenAPI 3.1, аутентификация и авторизация | Management Server | P0 | MVP | — | Proposed | |
| DLP-CORE-006 | RBAC с областями по `org_unit` | Management Server | P0 | MVP | — | Proposed | Модель ролей — кандидат U-03 |
| DLP-CORE-008 | Отзыв и ротация сертификатов агентов | Management Server | P1 | Пилот | — | Proposed | |
| DLP-CORE-009 | Отказоустойчивость сервера (active-passive, репликация PostgreSQL) | Management Server | P1 | Пилот | — | Proposed | Профиль HA — при окончательном решении о поставке (D-13 — предварительно) |
| DLP-CORE-010 | Резервное копирование и проверенное восстановление | Management Server | P1 | Пилот | — | Proposed | Защита копий — DLP-PLATFORM-002 |
| DLP-DETECTION-007 | Разбор архивов с лимитами глубины и распаковки | Detection Engine | P1 | Пилот | все | Proposed | Значения лимитов — Proposed (для агента — ADR-0018 §4; для прочих компонентов — при их проектировании); фаза не меняется (D-14) |
| DLP-DETECTION-008 | Обнаружение зашифрованных и запароленных файлов | Detection Engine | P1 | Пилот | все | Proposed | |
| DLP-DETECTION-009 | Exact Data Matching по хеш-индексу | Detection Engine | P1 | Пилот | все | Proposed | T-23 |
| DLP-DETECTION-010 | Document fingerprinting и частичное совпадение | Detection Engine | P1 | Пилот | все | Proposed | |
| DLP-DETECTION-011 | Чтение меток классификации из метаданных | Detection Engine | P1 | Пилот | все | Proposed | |
| DLP-DETECTION-012 | OCR изображений и сканированных PDF | Detection Engine | P2 | Production | сервер | Proposed | Только асинхронно |
| DLP-DETECTION-013 | Устаревшие форматы (DOC, XLS, RTF) | Detection Engine | P2 | Production | все | Proposed | |
| DLP-DETECTION-014 | ML/NLP-классификация вне блокирующего пути | Detection Engine | P3 | После Production | сервер | Proposed | |
| DLP-POLICY-007 | Версионирование, публикация с подтверждением, rollback | Policy Engine | P1 | MVP | — | Proposed | |
| DLP-POLICY-008 | Симуляция политики на исторических событиях | Policy Engine | P1 | Пилот | — | Proposed | |
| DLP-POLICY-009 | Режим Justification | Policy Engine | P2 | Пилот | Windows | Proposed | |
| DLP-POLICY-010 | Режим Quarantine | Policy Engine | P2 | Пилот | сеть, Discovery | Proposed | |
| DLP-ENDPOINT-001 | Сервис агента: жизненный цикл, конфигурация, журналы | Endpoint Agent | P0 | MVP | Windows (D-04) | Proposed | Разделение процессов и привилегий — D-15 (принцип); состав — после SPIKE-WIN-01 |
| DLP-ENDPOINT-002 | Minifilter: блокировка операции до записи на съёмный носитель (L2, без анализа содержимого) | Endpoint Agent | P0 | MVP | Windows (D-04) | Proposed | Первый кандидат BLOCK_CAPABLE — NOT VERIFIED до SPIKE-WIN-01; контентная блокировка — DLP-ENDPOINT-019 |
| DLP-ENDPOINT-003 | Белый список носителей по VID, PID и серийному номеру | Endpoint Agent | P0 | MVP | Windows (D-04) | Proposed | Не является строгой аутентификацией (T-26) |
| DLP-ENDPOINT-005 | Уведомление пользователя о блокировке | Endpoint Agent | P1 | MVP | Windows | Proposed | Помощник в сеансе пользователя без прав службы (D-15); TC-D15-02 |
| DLP-ENDPOINT-006 | Самозащита агента и проверка целостности | Endpoint Agent | P1 | Пилот | Windows | Proposed | |
| DLP-ENDPOINT-007 | Подписанное поэтапное обновление с откатом | Endpoint Agent | P1 | Пилот | все | Proposed | |
| DLP-ENDPOINT-008 | Агент Linux: fanotify, контроль USB через udev | Endpoint Agent | P1 | Пилот | Linux (D-04) | Proposed | |
| DLP-ENDPOINT-009 | Агент macOS: Endpoint Security | Endpoint Agent | P2 | Production | macOS (D-04) | Proposed | Entitlement Apple (R-02) |
| DLP-ENDPOINT-010 | Контроль печати | Endpoint Agent | P2 | Production | Windows, затем остальные | Proposed | MONITOR_ONLY до spike |
| DLP-ENDPOINT-011 | Мониторинг буфера обмена | Endpoint Agent | P2 | Production | Windows, macOS, X11 | Proposed | MONITOR_ONLY |
| DLP-ENDPOINT-012 | Расширение браузера для контроля загрузок | Endpoint Agent | P2 | Production | Chrome, Edge | Proposed | |
| DLP-ENDPOINT-013 | Управляемое удаление по токену с сервера | Endpoint Agent | P1 | Пилот | все | Proposed | |
| DLP-NETWORK-001 | SMTP-фильтр с проверкой вложений | Network Gateway | P1 | Пилот | — | Proposed | Пилот (D-12); Network PoC вне MVP — TC-D12-01 |
| DLP-NETWORK-002 | ICAP-сервер (REQMOD) | Network Gateway | P1 | Пилот | — | Proposed | Пилот (D-12); Network PoC вне MVP — TC-D12-01; RESPMOD — кандидат FID/DLP-NETWORK-004 |
| DLP-NETWORK-003 | Карантин писем с освобождением аналитиком | Network Gateway | P2 | Production | — | Proposed | |
| DLP-DISCOVERY-001 | Задания сканирования SMB и NFS | Data Discovery | P2 | Production | — | Proposed | |
| DLP-DISCOVERY-002 | Инкрементальное сканирование | Data Discovery | P2 | Production | — | Proposed | |
| DLP-DISCOVERY-003 | Отчёт о местонахождении чувствительных данных | Data Discovery | P2 | Production | — | Proposed | |
| DLP-INCIDENT-001 | Создание инцидентов и корреляция событий | Incident Management | P0 | MVP | — | Proposed | Через outbox (DLP-CORE-011) |
| DLP-INCIDENT-002 | Workflow: статусы, назначение, комментарии | Incident Management | P0 | MVP | — | Proposed | |
| DLP-CONSOLE-001 | Разделы Endpoints, Policies, Incidents, Audit | Web Console | P0 | MVP | — | Proposed | |
| DLP-CONSOLE-002 | System Health по реальным метрикам | Web Console | P1 | MVP | — | Proposed | |
| DLP-INTEGRATION-001 | Синхронизация пользователей и групп из AD/LDAP | Integrations | P0 | MVP | — | Proposed | |
| DLP-INTEGRATION-002 | SSO через OIDC, MFA для административных операций | Integrations | P1 | Пилот | — | Proposed | |
| DLP-INTEGRATION-003 | Экспорт событий и аудита в SIEM (syslog, CEF) | Integrations | P1 | Пилот | — | Proposed | Канал для DLP-CORE-015 |
| DLP-INTEGRATION-004 | SAML 2.0 | Integrations | P2 | Production | — | Proposed | |
| DLP-INTEGRATION-005 | Webhooks и интеграция с SOAR | Integrations | P2 | Production | — | Proposed | Защита от SSRF (T-25) |
| DLP-INTEGRATION-006 | Fortinet: ICAP-профиль FortiGate и syslog (после проверки документации) | Integrations | P2 | Пилот | — | Proposed | |

## 3. Кандидаты из feature/initial-documentation (не утверждены, без ID ARCH-0001)

Сопоставление всех 106 функций FID — RECON-0001 §6. Здесь перечислены функции, не покрытые разделами 1–2: ADAPT, DEFER и CONFLICT. Для каждой функции ADAPT 2026-11-10 проверены исходные требования и критерии приёмки по карточке FID. Перенос требует решения владельца; окончательный ID присваивается при включении.

### 3.1. ADAPT — предлагается перенести

| FID | Функция | Проверка критериев FID | Совместимость с утверждёнными решениями | Предлагаемое размещение |
| --- | --- | --- | --- | --- |
| FID/DLP-CORE-001 | Аутентификация операторов и MFA | Отказ административной операции без второго фактора; Argon2id; вход и отзыв сессии в аудите | Совместима; дополняет D-07 (повторная MFA для evidence) | Management Server |
| FID/DLP-CORE-002 | RBAC и разделение обязанностей | Тест запрета на каждый маршрут; аудитор только читает | Совместима с D-07 | DLP-CORE-006 + U-03 |
| FID/DLP-CORE-005 | Управление секретами | Сканер секретов; контролируемая ошибка без секрета; ротация без пересборки | Совместима | Платформа |
| FID/DLP-CORE-007 | Управление конфигурацией | Отказ запуска при невалидной конфигурации; TLS не отключается | Совместима | Платформа |
| FID/DLP-CORE-008 | Наблюдаемость | Сквозной идентификатор запроса; содержимое не попадает в логи | Совместима; критерий совпадает с DLP-PLATFORM-001 | Платформа |
| FID/DLP-CORE-011 | Безопасность цепочки поставки | Подпись и SBOM каждого артефакта; CI падает при нарушении порога | Совместима; реализация CI вне полномочий текущей задачи (D-10) | DevSecOps |
| FID/DLP-ENDPOINT-001 | Мониторинг файловой активности | Полный журнал операций, включая чтение, без пропусков | **Требует уточнения:** сплошной журнал противоречит минимизации D-07 и влияет на производительность. Предлагается ограничить операциями в области политики | Endpoint Agent |
| FID/DLP-ENDPOINT-013 | Централизованное управление агентами | Операции только через API и аудит; выведенный агент теряет доступ | Совместима | Management Server |
| FID/DLP-DETECTION-012 | Контекстная классификация | Снижение ложных срабатываний на корпусе; объяснимость | Совместима | Detection Engine |
| FID/DLP-POLICY-008 | Алгоритм разрешения конфликтов | Тест каждого конфликтного сочетания; объяснение показывает проигравшие правила | Совместима; **требует дополнения:** место INDETERMINATE в порядке строгости (D-05) | DLP-POLICY-002 + U-11 |
| FID/DLP-POLICY-009 | Исключения политик | Истёкшее исключение перестаёт действовать; аудит | Совместима; отличать от break-glass; истечение по монотонному времени (D-05) | Policy Engine |
| FID/DLP-NETWORK-004 | ICAP REQMOD и RESPMOD | Контрактные тесты RFC 3507; устойчивость к большим и неполным телам | Совместима; фаза — пилот (D-12) | Network Gateway |
| FID/DLP-DISCOVERY-003 | Классификация обнаруженных данных | Детерминированная метка; версия схемы | Совместима | Data Discovery |
| FID/DLP-DISCOVERY-004 | Анализ метаданных (владелец, права) | Отчёт об избыточно доступных файлах; время доступа не меняется | Совместима; учесть минимизацию D-07 | Data Discovery |
| FID/DLP-DISCOVERY-008 | Сканирование с разграничением доступа | Учётные данные не в логах; разграничение результатов | Совместима | Data Discovery |
| FID/DLP-INCIDENT-002 | Классификация серьёзности | Воспроизводимость; ручное изменение с причиной и аудитом | Совместима | Incident Management |
| FID/DLP-INCIDENT-007 | Назначение и конфликт интересов | Инцидент о самом аналитике ему не назначается | Совместима; U-03 | Incident Management |
| FID/DLP-INCIDENT-009 | Экспорт с манифестом хешей и подписью | Проверка целостности получателем; отдельное право | Совместима; двойной контроль массового экспорта — DLP-INCIDENT-006 | Incident Management |
| FID/DLP-CONSOLE-001 | Dashboard | Значения совпадают с API; при недоступности — ошибка, а не нули | Совместима | Web Console |
| FID/DLP-CONSOLE-003 | Пользователи и группы | Соответствие снимку каталога; время синхронизации | Совместима | Web Console |
| FID/DLP-CONSOLE-010 | Администрирование | Повторная MFA для опасных изменений; аудит | Совместима | Web Console |
| FID/DLP-INTEGRATION-008 | Публичный API и сервисные учётные записи | Токены со сроком и областью, хранятся как хеши, отзываются немедленно | Совместима | Management Server |
| FID/DLP-INTEGRATION-011 | Профили совместимости с прокси | Профиль = версия + настройки + автотест; неподтверждённые не заявляются | Совместима; согласуется с D-08 | Integrations |

### 3.2. DEFER — отложенные кандидаты

| FID | Функция | Причина |
| --- | --- | --- |
| FID/DLP-ENDPOINT-006 | Контроль передачи на сетевые ресурсы и через приложения | После подтверждения матрицы возможностей |
| FID/DLP-NETWORK-006 | Контроль облачных приложений по профилям прокси | Фаза Production; не CASB |
| FID/DLP-NETWORK-007 | Сбор сетевых событий от внешних источников | После пилота |
| FID/DLP-INCIDENT-004 | Оценка риска сотрудника | Требует правовой оценки (Q-04) |
| FID/DLP-CONSOLE-006 | Классификация данных в консоли | После Data Discovery |
| FID/DLP-CONSOLE-007 | Data Discovery в консоли | После Data Discovery |
| FID/DLP-CONSOLE-008 | Отчёты | После пилота |
| FID/DLP-CONSOLE-009 | Настройка интеграций в консоли | После пилота |
| FID/DLP-INTEGRATION-010 | Уведомления по SMTP | После пилота |
| FID/DLP-INTEGRATION-012 | Fortinet Security Fabric | Исследование официальных интерфейсов |

### 3.3. CONFLICT — решения приняты 2026-11-11

Категория CONFLICT сохранена для сверки с RECON-0001. Перенос кандидатов требует отдельного согласования.

| FID | Функция | Конфликт |
| --- | --- | --- |
| FID/DLP-CORE-010 | Поставка on-premises и air-gapped | C-02 → D-13 (предварительно): RPM/DEB + systemd; OCI не обязателен в MVP; изолированный контур — по Q-03 |
| FID/DLP-POLICY-005 | Политики каналов с автопонижением действия | C-06 → D-17: автопонижение отклонено; понижение только по исключению ADR-0019 (DLP-POLICY-015) |

## 4. Исключено из версии 1 (D-08)

| Маркер | Функция | Условие пересмотра |
| --- | --- | --- |
| X-01 | Снимки экрана | Отдельный ADR, правовая оценка, согласование |
| X-02 | Кейлоггинг | То же |
| X-03 | Специализированный перехват мессенджеров | То же |
| X-04 | CASB и прямое API-сканирование SaaS | То же |

## 5. Сводка

| Группа | Количество |
| --- | --- |
| Раздел 1: объём определён решениями D-03, D-05…D-08, D-14…D-17 | 41 |
| Раздел 2: проектные предложения ARCH-0001 | 48 |
| **Канонических функций всего** (66 из ARCH-0001 + 22 новые по D-05…D-08 + 1 новая по D-14) | **89** |
| Кандидаты FID: ADAPT / DEFER / CONFLICT | 23 / 10 / 2 |
| Исключено (D-08) | 4 |

## 6. История изменений

| Дата | Изменение | Change ID |
| --- | --- | --- |
| 2026-11-10 | Создан реестр. Перенесены 66 функций ARCH-0001; добавлены 20 функций по D-05…D-08; добавлены кандидаты FID и исключения D-08 | CHG-0003 |
| 2026-11-10 | Ревизия 2 по ревью: добавлена DLP-ENDPOINT-019 (L3, AR-06); уточнены DLP-ENDPOINT-002, -018, DLP-CORE-003, -011, DLP-POLICY-013, -014, DLP-INCIDENT-007 | CHG-0003 rev2 |
| 2026-11-10 | Ревизия 3 по ревью раунда 2: добавлена DLP-ENDPOINT-020 (исследование TPM, SR2-01); уточнены DLP-POLICY-013, DLP-ENDPOINT-014, -018, DLP-CORE-003, -011, DLP-POLICY-015 | CHG-0003 rev3 |
| 2026-11-11 | Ревизия 4 по ревью раунда 3: уточнена DLP-ENDPOINT-004 (маркеры в слоте первого неотправленного события, `IN_FLIGHT`, `TELEMETRY_INCOMPLETE`, SR3-02). Новые функции не добавлялись; статусы не менялись | CHG-0003 rev4 |
| 2026-11-11 | Решения D-12…D-17: добавлена DLP-ENDPOINT-021 (D-14); уточнены DLP-POLICY-012, -014, -015, DLP-ENDPOINT-015, DLP-INCIDENT-003 и примечания DLP-CORE-009, DLP-DETECTION-007, DLP-ENDPOINT-001, -005, DLP-NETWORK-001, -002, FID/DLP-NETWORK-004; раздел 3.3 — пометки решений. Статусы функций не менялись: все Proposed | CHG-0004 |
| 2026-11-13 | Этап 1 реализации `sentinel-core` (локально, не зафиксировано): DLP-POLICY-001, -002, -003, -011 — Proposed → In Progress по результатам TR-0001 (TEST_RESULTS.md). DLP-POLICY-012 и -015 затронуты частично (fallback и передача исключений ADR-0019), статус не менялся: ограничения компиляции и манифест возможностей не реализованы. Ни одна функция не Implemented: загрузчика бандла с подписью нет (DLP-POLICY-004 — Proposed) | CHG-0007 |
| 2026-11-13 | Решение владельца CHG7-OD2 (ИТ-10, вариант «а»): модель миров PDP v0.1 поддерживает только детекторы с порогом по количеству совпадений. Для DLP-DETECTION-005 (словари с весами) пороги по `score` в PDP v0.1 не поддерживаются и не могут интерпретироваться как пороги по количеству; поддержка — отдельный CHG; требование к загрузчику и компилятору — KNOWN_ISSUES.md KI-08. Статусы функций не менялись | CHG-0007 rev3.2 |
