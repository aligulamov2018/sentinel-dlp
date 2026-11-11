# Sentinel DLP — Project State

| Атрибут | Значение |
| --- | --- |
| Назначение | Текущее состояние проекта; читается в начале каждой сессии (CLAUDE.md §14) |
| Версия | 7 |
| Дата изменения | 2026-11-11 |
| Статус | Active |

## Current Phase
Architecture & Requirements (фаза 0)

## Project Status
Базовая архитектура частично согласована (D-01…D-11). Исходного кода нет. Ни одна функция не реализована: все функции в FEATURE_REGISTRY.md имеют статус Proposed. Начало разработки производственного кода запрещено до нового подтверждения владельца.

## Completed
- GitHub repository created
- README.md created
- CLAUDE.md engineering instructions added
- Архитектурный аудит ARCH-0001 (документ Claude Docs)
- Решения D-01…D-11 (DECISION_LOG.md)
- ADR-0001…0006, ADR-0016, ADR-0017 приняты (ADR-0005, 0006, 0016, 0017 — с ограничениями)
- Сравнительный аудит с веткой `feature/initial-documentation` — RECON-0001 (Draft, принят как рабочая основа)
- Документация CHG-0002 и CHG-0003 подготовлена **локально** в виде патча; в Git не зафиксирована
- Архитектурное ревью CHG-0002/CHG-0003 от 2026-11-10: Changes Requested; замечания AR-01…AR-09 учтены в ревизии 2 CHG-0003 (DECISION_LOG.md §7)
- Security Architecture Review, раунд 2: Changes Requested; замечания SR2-01…SR2-04 и уточнения 1–5 учтены в ревизии 3
- Security Architecture Review, раунд 3: Changes Requested; дизайн исправлений согласован 2026-11-11 (Approved for Documentation with Constraints); SR3-01…SR3-06 учтены в ревизии 4. Свойства алгоритма приёма проверены одноразовой моделью, семантика аренды — на локальном PostgreSQL 16 (не производственный код; механизмы остаются NOT VERIFIED)
- Финальное ревью ревизии 4 (2026-11-11): Conditionally Approved, Minor Changes Required; SR4-01 и SR4-02 учтены в ревизии 4.1; решения владельца по вопросам ревизии 4 — DECISION_LOG.md §7
- Final Architecture Approval (2026-11-11): CHG-0002 и CHG-0003 rev4.1 — Design Accepted; SR4-01, SR4-02 закрыты; редакционная корректировка внесена. Продуктовые механизмы — NOT VERIFIED; кода нет

## In Progress
- Согласование Git-операций для CHG-0002 и CHG-0003 rev4.1 по D-10 (отдельный процесс)

## Blocked / Waiting
- Решения владельца по C-01…C-06 (DECISION_PROPOSAL_C01_C06.md)
- Решения по кандидатам U-01…U-19 и функциям ADAPT, DEFER, CONFLICT
- Ответы на Q-01…Q-09
- Разрешение на push и PR — сейчас запрещено (D-10, уточнение от 2026-11-10)
- Проверка защиты веток и прав учётных данных GitHub — NOT VERIFIED (нет доступа к API)
- Сборка Rust и Go со сторонними зависимостями в текущей среде ИИ-ассистента невозможна: crates.io и proxy.golang.org недоступны (проверено 2026-11-10)

## Known Constraints
- Подпись драйвера Windows требует EV-сертификата и Microsoft Partner Center (R-01)
- Агент macOS требует entitlement Endpoint Security от Apple (R-02)
- Утверждения о механизмах перехвата ОС — NOT VERIFIED до spike (CAPABILITY_MATRIX.md)
- Контентная блокировка записи на USB — NOT VERIFIED (R-15)

## Missing Mandatory Documents (CLAUDE.md §7)
PROJECT_OVERVIEW.md, PRODUCT_REQUIREMENTS.md, FUNCTIONAL_REQUIREMENTS.md, NON_FUNCTIONAL_REQUIREMENTS.md, COMPONENT_CATALOG.md, SECURITY_ARCHITECTURE.md, API_SPECIFICATION.md, ENDPOINT_AGENT_DESIGN.md, POLICY_ENGINE_DESIGN.md, INTEGRATION_CATALOG.md, TEST_RESULTS.md, BACKLOG.md, KNOWN_ISSUES.md, TRACEABILITY_MATRIX.md. Частично их содержание есть в ветке `feature/initial-documentation` (кандидаты U-*).

## Next Steps
1. Отдельное согласование Git-операций по D-10 (push в `docs/arch-0001`, PR в `develop` без merge, предварительная проверка защиты веток и прав токена); ответы на остальные вопросы DECISION_LOG.md §8.
2. Решения по C-01…C-06.
3. После разрешения — push в `docs/arch-0001` и PR в `develop` по D-10, с предварительной проверкой защиты веток и прав токена.
4. Включение защиты `main` и `develop` (действие владельца репозитория).
5. Планы SPIKE-WIN-01, SPIKE-LNX-01, SPIKE-MAC-01; заявки на EV-сертификат и entitlement Apple.
