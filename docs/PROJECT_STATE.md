# Sentinel DLP — Project State

| Атрибут | Значение |
| --- | --- |
| Назначение | Текущее состояние проекта; читается в начале каждой сессии (CLAUDE.md §14) |
| Версия | 8 |
| Дата изменения | 2026-11-11 |
| Статус | Active |

## Current Phase
Architecture & Requirements (фаза 0)

## Project Status
Базовая архитектура частично согласована (D-01…D-17; D-13 — предварительно, D-14 и D-15 — в принципе). Исходного кода нет. Ни одна функция не реализована: все функции в FEATURE_REGISTRY.md имеют статус Proposed. Начало разработки производственного кода запрещено до нового подтверждения владельца.

## Completed
- GitHub repository created
- README.md created
- CLAUDE.md engineering instructions added
- Архитектурный аудит ARCH-0001 (документ Claude Docs)
- Решения D-01…D-11 (DECISION_LOG.md)
- ADR-0001…0006, ADR-0016, ADR-0017 приняты (ADR-0005, 0006, 0016, 0017 — с ограничениями)
- Сравнительный аудит с веткой `feature/initial-documentation` — RECON-0001 (Draft, принят как рабочая основа)
- Документация CHG-0002 и CHG-0003 подготовлена локально и опубликована одним коммитом в ветке `docs/arch-0001` (Draft PR #1); в `develop` не влита
- Архитектурное ревью CHG-0002/CHG-0003 от 2026-11-10: Changes Requested; замечания AR-01…AR-09 учтены в ревизии 2 CHG-0003 (DECISION_LOG.md §7)
- Security Architecture Review, раунд 2: Changes Requested; замечания SR2-01…SR2-04 и уточнения 1–5 учтены в ревизии 3
- Security Architecture Review, раунд 3: Changes Requested; дизайн исправлений согласован 2026-11-11 (Approved for Documentation with Constraints); SR3-01…SR3-06 учтены в ревизии 4. Свойства алгоритма приёма проверены одноразовой моделью, семантика аренды — на локальном PostgreSQL 16 (не производственный код; механизмы остаются NOT VERIFIED)
- Финальное ревью ревизии 4 (2026-11-11): Conditionally Approved, Minor Changes Required; SR4-01 и SR4-02 учтены в ревизии 4.1; решения владельца по вопросам ревизии 4 — DECISION_LOG.md §7
- Final Architecture Approval (2026-11-11): CHG-0002 и CHG-0003 rev4.1 — Design Accepted; SR4-01, SR4-02 закрыты; редакционная корректировка внесена. Продуктовые механизмы — NOT VERIFIED; кода нет
- Ограниченная публикация по D-10 (2026-11-11): Draft PR #1 из `docs/arch-0001` в `develop` (коммит `87c610be`, дерево `1defee0`, по данным GitHub API); не влит; merge и перевод в Ready for Review запрещены
- Решения по C-01…C-06 (2026-11-11): D-12…D-17; ADR-0018 и ADR-0019 приняты (ADR-0018 — в принципе)

## In Progress
- CHG-0004 (решения D-12…D-17): подготовлен **локально** в виде патча поверх дерева PR #1; ревизия 2 по архитектурному ревью от 2026-11-12 (CHG4-AR-01…03); в Git не зафиксирован, PR не создавался

## Blocked / Waiting
- Проверки по решениям D-12…D-16 (DECISION_LOG.md §3): Network PoC, серверные платформы и Q-03, механизм изоляции разбора и бенчмарк, механизм восстановления агента после SPIKE-WIN-01, бэкенд хранилища evidence к пилоту
- Статусы ADR-0007 и ADR-0014 — Proposed до отдельного решения владельца
- Решения по кандидатам U-01…U-19 и функциям ADAPT, DEFER, CONFLICT
- Ответы на Q-01…Q-09
- Git-операции для CHG-0004, изменение PR #1 и merge — запрещены без отдельного разрешения владельца
- Защита веток: ruleset `protect-main-develop` активен для `main` и `develop` (запрет удаления и non-fast-forward, обязательный PR, 0 обязательных одобрений — принято владельцем для работы одним разработчиком (DECISION_LOG.md §7), список обхода пуст) — прочитан через API при публикации PR #1 2026-11-11. Вебхуки, настройки Actions и установленные приложения — NOT VERIFIED
- Сборка Rust и Go со сторонними зависимостями в текущей среде ИИ-ассистента невозможна: crates.io и proxy.golang.org недоступны (проверено 2026-11-10)

## Known Constraints
- Подпись драйвера Windows требует EV-сертификата и Microsoft Partner Center (R-01)
- Агент macOS требует entitlement Endpoint Security от Apple (R-02)
- Утверждения о механизмах перехвата ОС — NOT VERIFIED до spike (CAPABILITY_MATRIX.md)
- Контентная блокировка записи на USB — NOT VERIFIED (R-15)

## Missing Mandatory Documents (CLAUDE.md §7)
PROJECT_OVERVIEW.md, PRODUCT_REQUIREMENTS.md, FUNCTIONAL_REQUIREMENTS.md, NON_FUNCTIONAL_REQUIREMENTS.md, COMPONENT_CATALOG.md, SECURITY_ARCHITECTURE.md, API_SPECIFICATION.md, ENDPOINT_AGENT_DESIGN.md (по D-15 — после SPIKE-WIN-01), POLICY_ENGINE_DESIGN.md, INTEGRATION_CATALOG.md, TEST_RESULTS.md, BACKLOG.md, KNOWN_ISSUES.md, TRACEABILITY_MATRIX.md. Частично их содержание есть в ветке `feature/initial-documentation` (кандидаты U-*).

## Next Steps
1. Архитектурное ревью локального патча CHG-0004; Git-операции — только по отдельному разрешению.
2. Решение владельца о merge PR #1; ответы на вопросы DECISION_LOG.md §8 и Q-01…Q-09 (прежде всего Q-03, Q-04, Q-05) и о серверных платформах.
3. Планы SPIKE-WIN-01 (включая песочницу разбора, замер IPC и сравнение механизмов восстановления), SPIKE-LNX-01, SPIKE-MAC-01; заявки на EV-сертификат и entitlement Apple.
4. Бенчмарк TC-D14-05 и выбор механизма изоляции разбора — после разрешения на начало разработки.
