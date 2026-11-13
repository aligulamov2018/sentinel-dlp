# Sentinel DLP — Project State

| Атрибут | Значение |
| --- | --- |
| Назначение | Текущее состояние проекта; читается в начале каждой сессии (CLAUDE.md §14) |
| Версия | 25 |
| Дата изменения | 2026-11-13 |
| Статус | Active |

## Current Phase
Architecture & Requirements (фаза 0) → начало реализации. CHG-0005 — Design Accepted и влит в `develop`. Этап 1 реализации `sentinel-core` (CHG-0007: workspace, `sentinel-core-types`, `sentinel-core-pdp`) выполнен **локально** по разрешению владельца от 2026-11-13; не зафиксирован в Git

## Project Status
Базовая архитектура частично согласована (D-01…D-17; D-13 — предварительно, D-14 и D-15 — в принципе). CHG-0002…CHG-0005 и запись о merge PR #3 находятся в `develop` (`c982a6f9`); `main` — `12e3d5c`. В Git production-кода нет. Локально (ветка `impl/sentinel-core-v0.1-local`, рабочее дерево без коммитов) реализованы и проверены типы контрактов ядра и PDP: `cargo test` 63/63, clippy и fmt — PASS, профиль CHG-0007 и его самопроверка — PASS (TEST_RESULTS.md TR-0005, проверки команды; ревьюер независимо подтвердил профиль rev3/rev3.1 62/62, самопроверку 23/23, пакет rev3.1 34/34 и дерево `298fec61`; результат профиля в TR-0002 получен дефектным профилем, CHG7-R4). DLP-POLICY-001, -002, -003, -011 — In Progress; остальные функции — Proposed; механизмы вне PDP — NOT VERIFIED. Толкования контракта: уточнения класса B и вариант «а» ИТ-10 утверждены владельцем (CHG7-OD1, CHG7-OD2, 2026-11-13) и применены к SENTINEL_CORE_DESIGN.md и FAILURE_MODE_DESIGN.md; score-пороги детекторов в PDP v0.1 не поддерживаются (KNOWN_ISSUES.md KI-08). Commit, push, PR и изменение настроек репозитория — только по отдельным решениям владельца.

## Completed
- GitHub repository created
- README.md created
- CLAUDE.md engineering instructions added
- Архитектурный аудит ARCH-0001 (документ Claude Docs)
- Решения D-01…D-11 (DECISION_LOG.md)
- ADR-0001…0006, ADR-0016, ADR-0017 приняты (ADR-0005, 0006, 0016, 0017 — с ограничениями)
- Сравнительный аудит с веткой `feature/initial-documentation` — RECON-0001 (Draft, принят как рабочая основа)
- Документация CHG-0002 и CHG-0003 подготовлена локально и опубликована одним коммитом в ветке `docs/arch-0001` (PR #1); влита в `develop` 2026-11-12
- Архитектурное ревью CHG-0002/CHG-0003 от 2026-11-10: Changes Requested; замечания AR-01…AR-09 учтены в ревизии 2 CHG-0003 (DECISION_LOG.md §7)
- Security Architecture Review, раунд 2: Changes Requested; замечания SR2-01…SR2-04 и уточнения 1–5 учтены в ревизии 3
- Security Architecture Review, раунд 3: Changes Requested; дизайн исправлений согласован 2026-11-11 (Approved for Documentation with Constraints); SR3-01…SR3-06 учтены в ревизии 4. Свойства алгоритма приёма проверены одноразовой моделью, семантика аренды — на локальном PostgreSQL 16 (не производственный код; механизмы остаются NOT VERIFIED)
- Финальное ревью ревизии 4 (2026-11-11): Conditionally Approved, Minor Changes Required; SR4-01 и SR4-02 учтены в ревизии 4.1; решения владельца по вопросам ревизии 4 — DECISION_LOG.md §7
- Final Architecture Approval (2026-11-11): CHG-0002 и CHG-0003 rev4.1 — Design Accepted; SR4-01, SR4-02 закрыты; редакционная корректировка внесена. Продуктовые механизмы — NOT VERIFIED; кода нет
- Ограниченная публикация по D-10 (2026-11-11): Draft PR #1 из `docs/arch-0001` в `develop` (коммит `87c610be`, дерево `1defee0`, по данным GitHub API)
- Решения по C-01…C-06 (2026-11-11): D-12…D-17; ADR-0018 и ADR-0019 приняты (ADR-0018 — в принципе)
- PR #1 влит владельцем 2026-11-12 merge-коммитом `322dade79ca872baae192a2c5a10807a1ce9dafc` (родители `12e3d5c`, `87c610be`; дерево `1defee0`)
- CHG-0004 rev2: Architecture Design Accepted; CHG4-AR-01…03 — Closed (Documentation); Release Candidate Accepted 2026-11-12
- CHG-0004 опубликован по D-10: коммит `448134651987511fc36621df82119562f7db83e5` в `docs/chg-0004`, PR #2; влит владельцем 2026-11-12 merge-коммитом `51422176cd3f56ecc62360f1549e402253a3bd07` (родители `322dade`, `448134`; дерево `17cbb22c8e7562a6e38cfa315435b1c28e40334e`). `main` не изменялся (`12e3d5c`)
- CHG-0005 — System Design `sentinel-core` v0.1 и `sentinel-scan`: **Design Accepted 2026-11-12 (OA-10)**; опубликован по D-10 (OA-11): коммит `94f7b97f86240d73f8f44cf7d9f4f96ed571bb3b` в `docs/arch-0001`, PR #3; PR #3 влит владельцем 2026-11-12 merge-коммитом `9ff53e507d41a18a7f6790e1f6296cbc30f41cd1` (родители `51422176`, `94f7b97f`; дерево `9d9bde6990860fa6337ac595ed4ed42432ede119`). `main` не изменялся (`12e3d5c`). Протокол приёмки — docs/CHG-0005-ACCEPTANCE-PROTOCOL.md; запись о публикации — DECISION_LOG.md §7. История: ревизия 13 — публикация и протокол приёмки в docs/CHG-0005-ACCEPTANCE-PROTOCOL.md; ревизия 12 — отметка Design Accepted; ревизия 11 от 2026-11-12 — отметки об утверждении владельцем; ревизия 10 — локальный интеграционный кандидат: CHG5-AR9-01, независимое ревью: Ready for Owner Approval; ревизия 9: CHG5-AR8-01; ревизия 8: CHG5-AR7-01, AR7-02, PKG-01; ревизия 7: CHG5-AR6-01, РВ-1…РВ-5, OBS-02…04; ревизия 6: CHG5-AR5-01…05; ревизия 5 — Target Design по проектам решений D-18…D-21 и уточнениям DPR-01…07; ранее CHG5-AR-01…08, CHG5-AR2-01…06, R4-01…14) подготовлен **локально** на базе `develop@51422176`; решения владельца OA-1…OA-9 от 2026-11-12 зафиксированы (D-18…D-21 Accepted; РВ-1…РВ-5, PG-1…PG-6, TA-1…TA-7, SENTINEL_CORE_DESIGN.md §6.8 утверждены; DECISION_LOG.md §1, §4.1, §7). Design Accepted — объект: пакет `CHG-0005-rev11-owner-decisions.zip`, дерево rev11 `6bf6c733`. Только документация

## In Progress
- CHG-0007 — этап 1 реализации `sentinel-core`: ревью rev3.2 принято владельцем (ревьюер независимо подтвердил 36/36, дерево `3a4009de`, профиль 63/63, самопроверку 23/23); решения CHG7-OD1, OD2 (B и C) и CHG7-OD6 (редакционная правка спецификаций) исполнены локально — rev3.3, TR-0006 (проверки команды); настройки GitHub (OD3) и публикация (OD4) отложены, риск PF-7 не принят. Проект разового разрешения D10-OP-CHG7 обновлён (PF-5/PV-2) и ожидает решения владельца
- TC-D10-06 — проект разделения учётных данных и ограничений веток (docs/security/REPO_ACCESS_RULESET_DESIGN.md): ожидает решений владельца по разделу 7; ничего не применено

## Blocked / Waiting
- Проверки по решениям D-12…D-16 (DECISION_LOG.md §3): Network PoC, серверные платформы и Q-03, механизм изоляции разбора и бенчмарк, механизм восстановления агента после SPIKE-WIN-01, бэкенд хранилища evidence к пилоту
- Статусы ADR-0007 и ADR-0014 — Proposed до отдельного решения владельца
- Решения по кандидатам U-01…U-19 и функциям ADAPT, DEFER, CONFLICT
- Ответы на Q-01…Q-09
- Git-операции: PR #3 и PR #4 влиты владельцем 2026-11-12; CHG-0007 не зафиксирован. Публикация отложена решением CHG7-OD4; ограничения D-10 сохраняются. Новые Git-операции — только по отдельному решению владельца
- ~~Формальное подтверждение владельцем проектов решений D-18…D-21~~ — получено 2026-11-12 (OA-1…OA-4; DECISION_LOG.md §1, §7)
- ~~Owner Approval изменений интеграционного кандидата rev7 (РВ-1…РВ-5)~~ — получено 2026-11-12 (OA-2 для РВ-2, OA-5…OA-8); публикация разрешена (OA-11)
- ~~Design Accepted CHG-0005 (OA-10)~~ — получено 2026-11-12 (DECISION_LOG.md §7); ~~публикация по D-10 (OA-11)~~ — разрешена 2026-11-12 (DECISION_LOG.md §7)
- Вопрос O-16 (отзыв и ротация `ReleaseTrustAnchors`) — до реализации `EnforcementReady`
- OBS-02…OBS-04 зафиксированы как ограничения реализации (SENTINEL_CORE_DESIGN.md §13.1); SEC-01…SEC-03 сверены с DPR-01, DPR-02, DPR-05/06 (§13.2) — подтверждение ревьюером
- Остальные открытые вопросы CHG-0005: O-03…O-10, O-12…O-14, S-02…S-05 (SENTINEL_CORE_DESIGN.md §12, SENTINEL_SCAN_CLI_DESIGN.md §8)
- Защита веток: ruleset `protect-main-develop` активен для `main` и `develop` (запрет удаления и non-fast-forward, обязательный PR, 0 обязательных одобрений — принято владельцем для работы одним разработчиком (DECISION_LOG.md §7), список обхода пуст) — прочитан через API при публикации PR #1 2026-11-11 и повторно при публикации PR #3 2026-11-12 (TC-D10-05). Вебхуки, настройки Actions и установленные приложения — NOT VERIFIED
- Сборка Rust и Go со сторонними зависимостями в текущей среде ИИ-ассистента невозможна: crates.io и proxy.golang.org недоступны (проверено 2026-11-10)

## Known Constraints
- Подпись драйвера Windows требует EV-сертификата и Microsoft Partner Center (R-01)
- Агент macOS требует entitlement Endpoint Security от Apple (R-02)
- Утверждения о механизмах перехвата ОС — NOT VERIFIED до spike (CAPABILITY_MATRIX.md)
- Контентная блокировка записи на USB — NOT VERIFIED (R-15)

## Missing Mandatory Documents (CLAUDE.md §7)
PROJECT_OVERVIEW.md, PRODUCT_REQUIREMENTS.md, FUNCTIONAL_REQUIREMENTS.md, NON_FUNCTIONAL_REQUIREMENTS.md, COMPONENT_CATALOG.md, SECURITY_ARCHITECTURE.md, API_SPECIFICATION.md, ENDPOINT_AGENT_DESIGN.md (по D-15 — после SPIKE-WIN-01), POLICY_ENGINE_DESIGN.md, INTEGRATION_CATALOG.md, BACKLOG.md, TRACEABILITY_MATRIX.md. TEST_RESULTS.md и KNOWN_ISSUES.md созданы 2026-11-13 (CHG-0007). Частично их содержание есть в ветке `feature/initial-documentation` (кандидаты U-*).

## Next Steps
1. Решения владельца: (а) публикация CHG-0007 по проекту разового разрешения D10-OP-CHG7 (ветка, состав файлов, один коммит, Draft PR в `develop`, без merge; риск PF-7 — принять или дождаться решения по настройкам GitHub); (б) настройки GitHub по разделу 7 docs/security/REPO_ACCESS_RULESET_DESIGN.md (CHG7-OD3, TC-D10-06).
2. Решения владельца по открытым вопросам CHG-0005; ответы на вопросы DECISION_LOG.md §8 и Q-01…Q-09 (прежде всего Q-03, Q-04, Q-05) и о серверных платформах.
3. Планы SPIKE-WIN-01 (включая песочницу разбора, замер IPC и сравнение механизмов восстановления), SPIKE-LNX-01, SPIKE-MAC-01; заявки на EV-сертификат и entitlement Apple.
4. После отдельного разрешения владельца на реализацию (Design Accepted получен) — реализация `sentinel-core` v0.1 и `sentinel-scan` по утверждённому проекту; бенчмарк TC-D14-05 и выбор механизма изоляции разбора.
