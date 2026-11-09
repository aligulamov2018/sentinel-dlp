# SENTINEL DLP — ENGINEERING INSTRUCTIONS

**Project:** Sentinel DLP — Enterprise Data Loss Prevention Platform  
**Project Type:** Enterprise Cybersecurity Software  
**Development Approach:** Architecture-First, Security-by-Design, Test-Driven Engineering  
**Primary Language:** Russian (documentation and communication)  
**Project Status:** Initial Architecture & Requirements Phase

---

# 1. PROJECT MISSION

Sentinel DLP — собственная корпоративная платформа Data Loss Prevention (DLP), предназначенная для обнаружения, предотвращения и расследования несанкционированной передачи конфиденциальной информации.

Цель проекта — создание полноценного, производительного, масштабируемого и безопасного программного продукта корпоративного уровня.

Система должна поддерживать on-premises развёртывание, централизованное управление, интеграцию с корпоративными системами идентификации и инфраструктурой информационной безопасности.

Проект не является демонстрационным приложением, учебным прототипом или экспериментом по vibe coding.

Главные инженерные приоритеты:

1. Security
2. Correctness
3. Reliability
4. Maintainability
5. Performance
6. Scalability
7. Testability
8. Documentation

Скорость разработки не должна достигаться за счёт снижения качества.

# 2. ENGINEERING ROLES

При выполнении задач действуй как междисциплинарная инженерная команда, включающая:

- Principal Software Architect
- Lead Backend Engineer
- Endpoint Security Engineer
- Application Security Engineer
- Data Protection Engineer
- Database Architect
- DevSecOps Engineer
- QA Automation Engineer
- Technical Writer
- Product Analyst

Критически оценивай инженерные решения.

Не соглашайся автоматически с технически некорректными предложениями.

Все значимые архитектурные решения должны быть обоснованы и документированы.

# 3. CORE DEVELOPMENT PRINCIPLES

## 3.1. Production-Oriented Engineering

Разрабатывай компоненты с учётом промышленной эксплуатации.

Предусматривай:

- Модульную архитектуру.
- Разделение ответственности.
- Минимизацию связности компонентов.
- Контролируемую обработку ошибок.
- Защищённое взаимодействие сервисов.
- Структурированное логирование.
- Мониторинг состояния компонентов.
- Возможность горизонтального масштабирования там, где это оправдано.
- Обратную совместимость API.
- Миграции баз данных.
- Управление конфигурациями.
- Автоматизированное тестирование.
- Контролируемое развёртывание и обновление.
- Процедуры восстановления после отказов.

Не усложняй архитектуру без необходимости.

Предпочитай обоснованные инженерные решения популярным технологиям.

## 3.2. No Fake Functionality

Строго запрещено представлять следующие элементы как реализованную функциональность:

- Mock API.
- Hardcoded dashboards.
- Fake incidents.
- Fake endpoint telemetry.
- Static security events.
- Неактивные кнопки.
- Фиктивные механизмы блокировки.
- Генераторы случайных событий вместо реального мониторинга.
- Заглушки без явной маркировки.

Mock-объекты разрешены в автоматизированных тестах и отдельно обозначенном демонстрационном режиме.

Каждая функция должна иметь подтверждаемую техническую реализацию.

## 3.3. Architecture Before Implementation

Перед разработкой нового компонента:

1. Изучи существующую архитектуру.
2. Проверь связанные требования.
3. Определи затрагиваемые подсистемы.
4. Проанализируй зависимости.
5. Оцени риски.
6. Предложи техническое решение.
7. Сформируй критерии приёмки.
8. Только затем приступай к реализации в рамках согласованной задачи.

Не выполняй масштабные рефакторинги без технического обоснования.

Не создавай дублирующую функциональность.

# 4. FUNCTIONAL ARCHITECTURE

Sentinel DLP должна предусматривать следующие подсистемы.

## 4.1. Endpoint DLP

Целевые платформы:

- Windows
- Linux
- macOS

Основные возможности:

- File Activity Monitoring.
- USB Device Control.
- Removable Media Control.
- Print Monitoring.
- Clipboard Monitoring.
- File Transfer Control.
- Endpoint Policy Enforcement.
- Local Policy Cache.
- Offline Event Buffering.
- Agent Health Monitoring.
- Agent Integrity Verification.
- Secure Agent Updates.
- Centralized Agent Management.

Для каждой ОС необходимо отдельно подтверждать доступность механизмов перехвата и блокировки.

Не обещай одинаковую функциональность Windows, Linux и macOS без технического подтверждения.

## 4.2. Content Inspection Engine

Предусмотри поддержку:

- Regular Expressions.
- Exact Data Matching.
- Document Fingerprinting.
- Partial Document Matching.
- Metadata Inspection.
- File Type Detection.
- Office Document Parsing.
- PDF Inspection.
- OCR.
- Archive Inspection.
- Keyword Dictionaries.
- Contextual Classification.
- Combined Detection Rules.
- ML/NLP Classification.

LLM и ML-компоненты должны быть дополнительными механизмами, а не обязательной зависимостью критически важных операций блокировки.

## 4.3. Policy Engine

Функциональность:

- Централизованное создание политик.
- Управление областями применения.
- Политики по пользователям и группам.
- Политики по устройствам.
- Политики по каналам передачи.
- Контекстные правила.
- Приоритеты политик.
- Разрешение конфликтов.
- Исключения.
- Версионирование.
- Аудит изменений.
- Rollback.
- Offline Policy Enforcement.

Решения Policy Engine должны быть детерминированными и объяснимыми.

## 4.4. Network DLP

Предусмотри:

- Email DLP.
- Интеграции с SMTP/MTA.
- HTTP(S) Proxy Integration.
- ICAP Integration.
- File Upload Inspection.
- Cloud Application Controls.
- Network Event Collection.
- Policy-Based Blocking.
- Quarantine.

Не утверждай, что система способна анализировать любое зашифрованное соединение без дополнительных технических механизмов и разрешений.

## 4.5. Data Discovery

- File Share Scanning.
- Sensitive Data Discovery.
- Data Classification.
- Metadata Analysis.
- Incremental Scanning.
- Classification Reports.
- Scheduled Scans.
- Access-Controlled Scanning.

## 4.6. Incident Management

- Incident Registration.
- Severity Classification.
- Event Correlation.
- Risk Scoring.
- Investigation Workflow.
- Evidence Management.
- Incident Assignment.
- Audit Trail.
- Incident Export.
- Retention Policies.
- SIEM/SOAR Integration.

## 4.7. Management Console

Разделы:

- Dashboard.
- Endpoints.
- Users and Groups.
- Policies.
- Incidents.
- Data Classification.
- Data Discovery.
- Reports.
- Integrations.
- Administration.
- Audit Logs.
- System Health.

Все рабочие интерфейсы должны взаимодействовать с реальными backend API.

## 4.8. Enterprise Integrations

- Active Directory.
- LDAP.
- SAML 2.0.
- OpenID Connect.
- Syslog.
- SIEM.
- SOAR.
- REST API.
- Webhooks.
- SMTP.
- Proxy.
- Fortinet Security Fabric при наличии официально поддерживаемых интерфейсов.

# 5. SECURITY REQUIREMENTS

При проектировании учитывай применимые положения:

- ISO/IEC 27001.
- ISO/IEC 27002.
- ISO/IEC 27005.
- NIST SP 800-53.
- NIST SP 800-218 SSDF.
- OWASP ASVS.
- MITRE ATT&CK.
- Zero Trust Architecture.
- Secure by Design.
- Privacy by Design.

Не заявляй о соответствии стандартам без подтверждающих проверок.

Обязательные механизмы безопасности:

- TLS.
- mTLS там, где обосновано.
- Encryption at Rest.
- RBAC.
- MFA для административных операций.
- Secure Secret Management.
- API Authentication.
- Authorization.
- Input Validation.
- Audit Logging.
- Certificate Lifecycle Management.
- Signed Updates.
- Dependency Vulnerability Scanning.
- Software Bill of Materials.
- Threat Modeling.

Для endpoint-агентов разрабатывай защиту от несанкционированного изменения конфигурации и отключения в пределах возможностей ОС и согласованной политики организации.

Мониторинг сотрудников допускается только в пределах законных полномочий, утверждённых политик безопасности и требований защиты персональных данных.

# 6. ARCHITECTURAL DECISIONS

Не выбирай технологический стек без предварительной оценки.

Проводи сравнительный анализ:

- Modular Monolith vs Microservices.
- Go vs Rust vs C++ vs C#.
- PostgreSQL и альтернативы.
- REST vs gRPC.
- Message Brokers.
- Object Storage.
- Search Engines.
- Containerization.
- Kubernetes/OpenShift.
- On-Premises Deployment.
- Air-Gapped Deployment.

Каждое значимое архитектурное решение фиксируй в ADR.

ADR должен содержать:

- Контекст.
- Рассмотренные варианты.
- Принятое решение.
- Обоснование.
- Последствия.
- Риски.
- Дату.
- Статус.

# 7. MANDATORY DOCUMENTATION

Документация является обязательной частью системы.

Поддерживай каталог docs/ со следующими файлами:

- PROJECT_OVERVIEW.md
- PRODUCT_REQUIREMENTS.md
- FUNCTIONAL_REQUIREMENTS.md
- NON_FUNCTIONAL_REQUIREMENTS.md
- SYSTEM_ARCHITECTURE.md
- COMPONENT_CATALOG.md
- FEATURE_REGISTRY.md
- SECURITY_ARCHITECTURE.md
- THREAT_MODEL.md
- DATA_MODEL.md
- API_SPECIFICATION.md
- ENDPOINT_AGENT_DESIGN.md
- POLICY_ENGINE_DESIGN.md
- INTEGRATION_CATALOG.md
- TEST_STRATEGY.md
- TEST_RESULTS.md
- ROADMAP.md
- BACKLOG.md
- CHANGELOG.md
- DECISION_LOG.md
- KNOWN_ISSUES.md
- PROJECT_STATE.md
- REFERENCE_ANALYSIS.md
- TRACEABILITY_MATRIX.md

Не заполняй документы вымышленными фактами.

Документация должна соответствовать фактическому состоянию исходного кода.

# 8. FEATURE REGISTRY

Все функции системы регистрируй в FEATURE_REGISTRY.md.

Для каждой функции указывай:

- Feature ID.
- Feature Name.
- Description.
- Priority.
- Functional Requirements.
- Responsible Component.
- Dependencies.
- API.
- Data Model.
- Implementation Status.
- Acceptance Criteria.
- Test References.
- Known Limitations.
- Change History.

Используй уникальные идентификаторы:

- DLP-CORE-001.
- DLP-ENDPOINT-001.
- DLP-POLICY-001.
- DLP-DETECTION-001.
- DLP-INCIDENT-001.
- DLP-NETWORK-001.
- DLP-INTEGRATION-001.

Допустимые статусы:

- Proposed.
- Designed.
- In Progress.
- Implemented.
- Tested.
- Integrated.
- Production Ready.
- Deprecated.

Наличие пользовательского интерфейса не означает, что функция реализована.

# 9. CHANGE MANAGEMENT

Каждое значимое изменение должно сопровождаться записью в CHANGELOG.md.

Фиксируй:

1. Change ID.
2. Дату изменения.
3. Описание изменения.
4. Причину.
5. Изменённые компоненты.
6. Связанные требования.
7. Влияние на API.
8. Влияние на базу данных.
9. Риски.
10. Результаты тестов.
11. Совместимость.
12. Процедуру отката.
13. Итоговый статус.

Не удаляй историю изменений.

Документацию обновляй вместе с исходным кодом.

# 10. QUALITY ASSURANCE

Для каждого компонента определяй применимые проверки:

- Build Validation.
- Static Analysis.
- Linting.
- Unit Tests.
- Integration Tests.
- Contract Tests.
- Security Tests.
- Regression Tests.
- Performance Tests.
- Failure Recovery Tests.

Не удаляй тесты ради успешного результата CI.

Не утверждай, что тесты пройдены, если они не запускались.

Если проверка невозможна, указывай NOT VERIFIED.

# 11. DEFINITION OF DONE

Функция считается завершённой только после:

1. Регистрации требований.
2. Реализации программного кода.
3. Проверки корректности.
4. Выполнения обязательных тестов.
5. Проверки безопасности.
6. Обновления Feature Registry.
7. Обновления Changelog.
8. Актуализации архитектурной документации.
9. Регистрации известных ограничений.
10. Подготовки технического отчёта.

Production Ready требует дополнительных эксплуатационных и приёмочных испытаний.

# 12. GIT WORKFLOW

Используй следующую модель ветвления:

- main — стабильная основная ветка.
- develop — интеграционная ветка.
- feature/* — разработка новых возможностей.
- fix/* — исправление дефектов.
- release/* — подготовка выпусков.

Правила:

- Не выполняй force push.
- Не изменяй историю Git без явного разрешения.
- Не отправляй код в main без Pull Request.
- Не выполняй commit, push или merge без согласия пользователя.
- Не сохраняй секреты и конфиденциальную информацию в репозитории.
- Перед фиксацией показывай список изменённых файлов.
- По возможности проводи code review и выполняй CI.

# 13. REFERENCE REPOSITORIES

Внешние open-source репозитории рассматривай как технические референсы.

Перед повторным использованием:

1. Изучи лицензию.
2. Проверь архитектуру.
3. Проанализируй зависимости.
4. Оцени безопасность.
5. Проверь фактическую реализацию функций.
6. Изучи тестовое покрытие.
7. Оцени технический долг.
8. Подготовь REFERENCE_ANALYSIS.md.

Не копируй компоненты без проверки совместимости лицензий.

Не смешивай референсные исходники с собственным кодом без документирования происхождения.

# 14. SESSION CONTINUITY

В начале каждой сессии:

1. Прочитай CLAUDE.md.
2. Прочитай docs/PROJECT_STATE.md.
3. Прочитай docs/FEATURE_REGISTRY.md.
4. Прочитай docs/SYSTEM_ARCHITECTURE.md.
5. Проверь состояние Git-репозитория.
6. Определи незавершённые задачи.

В конце каждой задачи:

1. Обнови техническую документацию.
2. Обнови статусы функций.
3. Обнови CHANGELOG.md.
4. Обнови PROJECT_STATE.md.
5. Зарегистрируй оставшиеся проблемы.
6. Подготовь технический отчёт.
7. Предложи следующий шаг.

Если файл отсутствует, сначала сообщи об этом и при необходимости создай его.

Не заявляй, что информация сохранена, если фактической записи в файл не было.

# 15. TASK REPORT FORMAT

После каждой задачи предоставляй отчёт:

## Task
Название и идентификатор.

## Requirements
Какие требования выполнены.

## Architecture Impact
Какие компоненты затронуты.

## Implementation
Что реализовано.

## Files Changed
Какие файлы добавлены или изменены.

## Validation
Какие тесты запускались и их результаты.

## Security Impact
Влияние на безопасность.

## Documentation Updates
Какие документы обновлены.

## Outstanding Issues
Незавершённые вопросы.

## Next Step
Следующая рекомендуемая задача.

# 16. INITIAL DEVELOPMENT PHASE

Текущий этап: Architecture & Requirements.

На первоначальном этапе:

1. Изучи требования Sentinel DLP.
2. Изучи доступные референсные репозитории.
3. Подготовь Product Requirements.
4. Определи архитектурные варианты.
5. Сформируй функциональный каталог.
6. Разработай предварительную модель угроз.
7. Предложи технологический стек.
8. Подготовь Roadmap.
9. Сформируй требования к тестированию.
10. Документируй риски и ограничения.

Не начинай массовую генерацию исходного кода до согласования архитектуры.

**Основной принцип: разработка проверяемой, безопасной и сопровождаемой Enterprise DLP, а не быстрая генерация программных файлов.**
