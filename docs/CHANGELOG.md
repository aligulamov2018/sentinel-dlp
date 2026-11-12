# Sentinel DLP — Changelog

| Атрибут | Значение |
| --- | --- |
| Назначение | История всех значимых изменений (CLAUDE.md §9) |
| Версия | 5 |
| Дата изменения | 2026-11-12 |
| Статус | Active |

Записи не удаляются. Новые записи добавляются сверху. Идентификаторы `CHG-NNNN` уникальны и повторно не используются.

## CHG-0005 — System Design `sentinel-core` v0.1 и `sentinel-scan` (Design Accepted 2026-11-12; ранее Design Draft)

| Поле | Значение |
| --- | --- |
| Change ID | CHG-0005 |
| Дата и версия | 2026-11-12; Design Draft 0.1, ревизия 2 (0.2), ревизия 3 (0.3), ревизия 4 (0.4), ревизия 5 (0.5), ревизия 6 (0.6), ревизия 7 (0.7, интеграционный кандидат), ревизия 8 (0.8), ревизия 9 (0.9), ревизия 10 (0.10), ревизия 11 (0.11, отметки об утверждении), ревизия 12 (0.12, Design Accepted) и ревизия 13 (публикация по OA-11) — 2026-11-12; версия продукта не присвоена (кода нет). База — `develop@51422176cd3f56ecc62360f1549e402253a3bd07`, дерево `17cbb22c8e7562a6e38cfa315435b1c28e40334e` |
| Инициатор | Владелец продукта: разрешение на CHG-0005 (design-only) от 2026-11-12 |
| Что было | Проект `sentinel-core` и `sentinel-scan` отсутствовал; PROJECT_STATE.md и записи CHG-0003 и CHG-0004 в этом журнале не отражали публикацию и merge PR #1 и PR #2 |
| Что изменено | Добавлены docs/architecture/SENTINEL_CORE_DESIGN.md, docs/architecture/SENTINEL_SCAN_CLI_DESIGN.md, docs/architecture/SENTINEL_CORE_TEST_PLAN.md (тест-кейсы TC-CORE-01…45, TC-SCAN-01…15, golden-ожидания G-01…G-30 — Not Run). Обновлены PROJECT_STATE.md (состояние после merge PR #1 и PR #2, текущая стадия) и этот журнал (публикация и merge CHG-0002…CHG-0004). Ревизия 5 — узкие изменения FAILURE_MODE_DESIGN.md §2.1, §3, §6.1, TEST_STRATEGY.md (TC-D05-08, TC-SEC-02), DECISION_LOG.md §4.1 (проекты D-18…D-21) по разрешению владельца. Ревизия 7 (интеграционный кандидат, Pending Owner Approval) — ADR-0018 §5 и риск T-14, FAILURE_MODE_DESIGN.md §2.2 и FI-D05-08, TEST_STRATEGY.md (TC-D12-01, TC-D14-01, TC-D14-02, TC-SEC-04), THREAT_MODEL.md T-14 |
| Причина | Проектирование первого компонента по D-03 до начала реализации; актуализация состояния проекта |
| Затронутые компоненты | Только документация: Detection Engine (`sentinel-core`), Policy Engine (PDP), CLI `sentinel-scan` |
| Связанные требования | D-03, D-05, D-07, D-14, D-16, D-17; D-18…D-21 (Accepted 2026-11-12, ранее проекты); DLP-DETECTION-001…006, -015; DLP-POLICY-001…006, -011…015; DLP-ENDPOINT-021 |
| Изменения API / БД | Нет реализованных изменений. Проект предлагает контракты API ядра и вывода CLI. Ревизия 5: новые поля и значения `InspectionResult`, `Decision` и записи бандла (`IncompleteInspectionAction`, `PartInspectionExclusion`, `PrecedenceConflictApproval`) — спецификация зависимостей SENTINEL_CORE_DESIGN.md §12.1. DATA_MODEL.md не изменялся |
| Риски | Открытые вопросы O-03…O-10, O-12…O-14 и S-02…S-05; проекты решений D-18…D-21 ожидали подтверждения (утверждены 2026-11-12, OA-1…OA-4); производительность (R-11), выход за границу песочницы (R-25), утечки через вывод (T-21), ложные срабатывания (R-05). Все механизмы — NOT VERIFIED |
| Совместимость | D-01…D-17, ADR-0016, ADR-0019 и CLAUDE.md не изменены; ADR-0018 изменён только в интеграционном кандидате rev7 (раздел 5, риск T-14; утверждено владельцем 2026-11-12, OA-5); изменения принятых документов — только в разрешённых владельцем разделах |
| Миграции | Нет |
| Результаты проверок | Документальные проверки выполнены локально скриптами вне репозитория: структура и перекрёстные ссылки документов (идентификаторы TC, O, S, R, T, ссылки на разделы), целевые проверки исправлений ревизий 2, 3, 4 и 5, рендеринг диаграмм Mermaid, одноразовая модель алгоритма SENTINEL_CORE_DESIGN.md §6.3 в модели независимых счётчиков (golden G-01…G-21, свойства на 20 000 случайных политиках), применимость патча к `develop@51422176` (`git apply --check`). Продуктовые тесты не применимы: кода нет; все TC — Not Run |
| Откат | До фиксации — не применять патч. После фиксации — `git revert` |
| Статус | **Design Accepted — 2026-11-12 (OA-10)**; ревизия 13 — ограниченная публикация по D-10 разрешена (OA-11, 2026-11-12): один документационный коммит в `docs/arch-0001`, Draft PR в `develop`; merge не разрешён. Ранее: ревизия 11 — решения владельца OA-1…OA-9 (D-18…D-21 Accepted; РВ-1…РВ-5, PG-1…PG-6, TA-1…TA-7 и SENTINEL_CORE_DESIGN.md §6.8 утверждены); независимое ревью rev10 — Ready for Owner Approval. Реализация — только по отдельному разрешению владельца. Продуктовые тесты — Not Run; механизмы — NOT VERIFIED; алгоритм — Model Verified; числовые параметры — Proposed |

### CHG-0005, ревизия 13 — ограниченная публикация по D-10 (OA-11, 2026-11-12)

Основание: разрешение владельца продукта OA-11 от 2026-11-12. Объект публикации — пакет `CHG-0005-rev12-design-accepted.zip` (SHA-256 `245364ce4ef99f1fa0525947b6172b219e5f82f7d86ad057bac6a21c98fc829f`), дерево rev12 `72c71841986d6a753f758bea983a1a6469b91f2c`, протокол приёмки v1.3 (SHA-256 `46bc7cbd922f7eccc53440b0230ff5427877ddb34a0de8f4d85511c2e3ab24c8`).

| Что было | Что изменено | Где |
| --- | --- | --- |
| Протокол приёмки v1.3 — вне репозитория | Добавлен протокол v1.4: решение OA-11, раздел о публикации и изменениях относительно дерева rev12 | docs/CHG-0005-ACCEPTANCE-PROTOCOL.md (новый) |
| Отметки «в Git не опубликовано (OA-11)» | «публикация по D-10 разрешена 2026-11-12 (OA-11; Draft PR в develop)» | Шапки ADR-0018, FAILURE_MODE_DESIGN.md, TEST_STRATEGY.md, THREAT_MODEL.md; SENTINEL_CORE_DESIGN.md — шапка (версия 0.13), разделы 12.2, 13.3 |
| OA-11 не записан | Запись «Ограниченная публикация CHG-0005 по D-10 (OA-11)» в DECISION_LOG.md §7 | DECISION_LOG.md (версия 6.8) |
| PROJECT_STATE — ожидание OA-11 | Публикация разрешена; Draft PR в develop; merge — решение владельца | PROJECT_STATE.md (версия 18) |

Нормативное содержание принятого проекта не изменено (diff rev12 → rev13: только отметки, записи и протокол). SENTINEL_SCAN_CLI_DESIGN.md, SENTINEL_CORE_TEST_PLAN.md, DATA_MODEL.md, CLAUDE.md, D-01…D-21 не изменены. SHA коммита и номер PR вносятся следующим изменением документации. Сохраняются: Design Accepted; продуктовые тесты — Not Run; механизмы — NOT VERIFIED; PDP — Model Verified в документированных границах; числовые параметры — Proposed. Откат — закрыть Draft PR без merge; после merge — `git revert`.

### CHG-0005, ревизия 12 — Design Accepted (OA-10, 2026-11-12)

Основание: решение владельца продукта OA-10 от 2026-11-12. Объект принятия — архитектурное содержание rev10 с фиксацией OA-1…OA-9 в rev11: пакет `CHG-0005-rev11-owner-decisions.zip` (SHA-256 `45342e3c5a8b39bc0f487898def0dbf96300caf35f2210ddc408c3e129d311bd`), дерево rev11 `6bf6c733a2afa24b59916529bbad114b1b7cf58f`. Ревьюер независимо проверил фиксацию OA-1…OA-9 в rev11; блокирующих замечаний нет.

| Что было | Что изменено | Где |
| --- | --- | --- |
| CHG-0005 — Design Draft, ожидает Design Accepted (OA-10) | Design Accepted 2026-11-12 (OA-10) | SENTINEL_CORE_DESIGN.md, SENTINEL_SCAN_CLI_DESIGN.md, SENTINEL_CORE_TEST_PLAN.md — шапки (версия 0.12, статус); SENTINEL_CORE_DESIGN.md — разделы 6.3, 13.3; SENTINEL_SCAN_CLI_DESIGN.md — раздел 9 |
| OA-10 не записан | Запись «Design Accepted CHG-0005 (OA-10)» в DECISION_LOG.md §7; предыдущая запись OA-1…OA-9 не изменена | DECISION_LOG.md (версия 6.7) |
| Заголовок записи CHG-0005 «(Design Draft)» | «(Design Accepted 2026-11-12; ранее Design Draft)»; статус записи | Этот журнал |

Нормативное содержание не изменено: правки — только отметки статуса (diff rev11 → rev12). ADR, FAILURE_MODE_DESIGN.md, TEST_STRATEGY.md, THREAT_MODEL.md, DATA_MODEL.md, CLAUDE.md, D-01…D-21 не изменены. Сохраняются: продуктовые тесты — Not Run; механизмы — NOT VERIFIED; алгоритм — Model Verified в документированных границах; числовые параметры — Proposed. Публикация (OA-11) и реализация — отдельные решения. Откат — не применять diff rev11 → rev12.

### CHG-0005, ревизия 11 — фиксация решений владельца OA-1…OA-9 (2026-11-12)

Основание: независимое ревью rev10 — Ready for Owner Approval (CHG5-AR9-01 закрыт, PKG-01 и воспроизводимость деревьев подтверждены); протокол приёмки CHG-0005 v1.1; формальное подтверждение владельца продукта от 2026-11-12. Объект утверждения — пакет `CHG-0005-rev10-review.zip` (SHA-256 `e02405c96d1ca5df721f9c07212964cd382848a7518b9aa04d7051a49f468668`), дерево rev10 `012e02a1554d8af72974f38603cc03fd8210f4a2`.

| Что было | Что изменено | Где |
| --- | --- | --- |
| D-18…D-21 — проекты (Proposed) в DECISION_LOG.md §4.1 | Accepted 2026-11-12 (OA-1…OA-4), перенесены в §1; таблица §4.1 сохранена как история и перечень условий; запись ревью и решений в §7 | DECISION_LOG.md (версия 6.6) |
| РВ-1…РВ-5 — интеграционный кандидат, «вступает после Owner Approval» | Отметки: утверждено владельцем (OA-2 для РВ-2; OA-5…OA-8); в Git не опубликовано (OA-11) | ADR-0018, FAILURE_MODE_DESIGN.md, TEST_STRATEGY.md, THREAT_MODEL.md (шапки и пометки Target Design); SENTINEL_CORE_DESIGN.md §12.2 |
| Target Design (Proposed) по проектам D-18…D-21; PG-1…PG-6, TA-1…TA-7 и SENTINEL_CORE_DESIGN.md §6.8 — Proposed | Target Design по утверждённым решениям; требования утверждены (OA-9); числовые значения — Proposed | SENTINEL_CORE_DESIGN.md: шапка, разделы 4.1, 4.2, 6.3, 6.4, 6.7.1, 6.8, 12, 12.2, 13.3; SENTINEL_SCAN_CLI_DESIGN.md: шапка, разделы 7, 8; SENTINEL_CORE_TEST_PLAN.md: шапка, TC-CORE-11, раздел 5 |

Нормативное содержание ревизии 10 не изменено: правки затрагивают только отметки статуса и утверждения (CHG-0005 rev11, diff rev10 → rev11). D-01…D-17, ADR-0016, ADR-0019, DATA_MODEL.md и CLAUDE.md не изменены. Продуктовые тесты — Not Run; механизмы — NOT VERIFIED; алгоритм — Model Verified в документированных границах. Откат — не применять diff rev10 → rev11.

### CHG-0005, ревизия 10 — узкое исправление CHG5-AR9-01 (2026-11-12)

Основание: независимое review rev9 — замечание CHG5-AR9-01. Новый полный цикл не запускался.

| ID | Что было | Что сделано | Где | Тесты |
| --- | --- | --- | --- | --- |
| CHG5-AR9-01 | В пути `Engine::evaluate` хост заранее отключал записи, истекающие до дедлайна запроса, а повторное `Pdp::decide` выполнял только при смене состояния времени. По F-08 упреждающее отключение `PartInspectionExclusion` смягчало результат до фактического истечения: E1 до t = 105, дедлайн 130, PDP при t = 101, правило BLOCK при NOT `count(card) >= 1` — вместо BLOCK получался INDETERMINATE с fallback ALLOW_WITH_AUDIT | Финальное решение в обоих путях API — вызов `Pdp::decide` со входом, вычисленным хостом непосредственно перед вызовом по всем записям-отступлениям бандла, включая записи, недействительные при формировании запроса. Упреждение по дедлайну отменено. `Engine::evaluate` возвращает `Evaluation { inspection, provisional: ProvisionalDecision }`; предварительное решение — отдельный тип: не применяется, не записывается, не проецируется. Все поля решения и проекция `DisclosedFindings` — из финального вызова. INV-H3 переформулирован; отложенное применение требует повторного финального вызова. Обработка `Unavailable` и `CONTRADICTORY` через отсутствие действительных отступлений сохранена; утверждение, что она всегда безопаснее fallback хоста, удалено как недоказанное. INV-C15 уточнён: недействительное исключение не участвует в решении, результат может быть мягче, чем с записью | SENTINEL_CORE_DESIGN.md §2.3, §3, §5, §6.3, §6.8, §10.3 (INV-C15), §12.1, §13, §13.3; SENTINEL_CORE_TEST_PLAN.md; SENTINEL_SCAN_CLI_DESIGN.md §2, §4.2 | TC-CORE-45 (обновлён); G-26…G-30 |

D-01…D-17, ADR, FAILURE_MODE_DESIGN.md, TEST_STRATEGY.md, THREAT_MODEL.md, DECISION_LOG.md и DATA_MODEL.md в rev10 не изменены. D-18…D-21 — Proposed. Продуктовые тесты — Not Run; механизмы — NOT VERIFIED; алгоритм — Model Verified с указанной границей независимости; протокол финального решения в модели выполняется по построению.

### CHG-0005, ревизия 9 — узкое исправление CHG5-AR8-01 (2026-11-12)

Основание: независимое review rev8 — предварительно Changes Requested (пакет `b269841d…`). Новый полный цикл не запускался.

| ID | Что было | Что сделано | Где | Тесты |
| --- | --- | --- | --- | --- |
| CHG5-AR8-01 | `DeviationValidity` входила только в `EvaluationRequest`; у раздельного `Pdp::decide(policy, context, inspection)` входа действительности не было; момент вычисления не определён | `Pdp::decide` принимает `&DeviationValidity`; тип — `Provided { time_state, valid_ids }` или `Unavailable`. Хост вычисляет вход непосредственно перед `Pdp::decide`; для `Engine::evaluate` — действительность на весь интервал до дедлайна и повторное `Pdp::decide` при смене состояния времени. Отсутствующий вход → все отступления недействительны (`UNAVAILABLE`); противоречивый → все недействительны (`CONTRADICTORY`); `EngineFault` не возникает. INV-H3 уточнён | SENTINEL_CORE_DESIGN.md §2.3, §5, §6.8, §12.1; SENTINEL_SCAN_CLI_DESIGN.md §2, §4.2 | TC-CORE-45 (Contract, Negative, Fault injection); G-25 |
| Редакция SENTINEL_CORE_DESIGN.md §6.8 (F-08) | «Обе записи разрешают исход, который без них был бы строже» — неверно для `PartInspectionExclusion` | Разделено: утверждение — разрешающее; исключение меняет интерпретацию неизвестного; гарантия для недействительного исключения — семантика отсутствующей записи; для срока действия обе записи учитываются как разрешающие отступления (консервативно) | SENTINEL_CORE_DESIGN.md §6.8 | G-24 |
| Граница Model Verified | Независимость оракула не оговорена | Оракул использует общие с алгоритмом функции конфликтов, подавления и выбора победителя; логику конфликтов независимо проверяют только заданные вручную ожидания | SENTINEL_CORE_DESIGN.md §13, §13.3; SENTINEL_CORE_TEST_PLAN.md §5 | — |
| Пакет | Ревьюер не смог воспроизвести деревья из-за среды Git | В пакет добавлены проверка без Git (чистый Python: применение патчей и вычисление Git tree SHA) и деревья в tar | Пакет review | verify_nogit.py |

D-01…D-17, ADR, FAILURE_MODE_DESIGN.md, TEST_STRATEGY.md, THREAT_MODEL.md и DECISION_LOG.md в rev9 не изменены. D-18…D-21 — Proposed. Продуктовые тесты — Not Run; механизмы — NOT VERIFIED; алгоритм — Model Verified с указанной границей независимости.

### CHG-0005, ревизия 8 — узкие исправления CHG5-AR7-01, AR7-02, PKG-01 (2026-11-12)

Основание: независимый аудит rev7 — Changes Requested, только узкие исправления (полный патч rev7 `412d31b4…`). Новый полный цикл не запускался.

| ID | Что было | Что сделано | Где | Тесты |
| --- | --- | --- | --- | --- |
| CHG5-AR7-01 | Срок действия `PartInspectionExclusion` и `PrecedenceConflictApproval` во время работы, в офлайне и при откате времени не определён | Записи — разрешающие отступления по FAILURE_MODE_DESIGN.md §4.5–§4.6: `expires_at`, `max_offline_validity`; действительность вычисляет хост (`DeviationValidity`, INV-H3); недействительное исключение не применяется, недействительное утверждение подавляет пару, fallback не ослабляется (INV-C15). Инварианты INV-T1…T5 не меняются | SENTINEL_CORE_DESIGN.md §2.3, §4.2, §6.1, §6.3, §6.8, §10.3; SENTINEL_SCAN_CLI_DESIGN.md §2, §4.2 | TC-CORE-43, TC-SCAN-15; G-24 |
| CHG5-AR7-02 | Достоверное затенение BLOCK правилом только по контексту не требовало утверждения: классы (a) и (b) его не охватывали | Конфликт — любая пара h ≻ l (h — MONITOR или ALERT, l — BLOCK) с пересекающейся областью; класс (c); утверждение с ограниченной областью; во время работы — подавление без действительного утверждения; порядок правил не меняется | SENTINEL_CORE_DESIGN.md §6.3 шаг 2, §6.7, INV-C12 | TC-CORE-35, TC-CORE-44; G-23 |
| PKG-01 | Отдельно приложенный SENTINEL_SCAN_CLI_DESIGN.md (blob `05315429…`) — копия из пакета rev5; в пакет rev7 актуальный файл не был отправлен | Канонический источник — полный патч; все отдельные файлы пакета пересобраны из применённого полного патча и сверены по Git blob SHA | Пакет rev8 | Сверка blob SHA |
| Редакция SENTINEL_CORE_DESIGN.md §4.3 | Обещание «ни в одном случае не `NO_MATCH`» для ZIP с префиксом противоречило признанному остаточному риску; TC-CORE-34 ожидал отсутствие `NO_MATCH` для UTF-16 | Обещание ограничено случаями, когда срабатывают PG-1 или PG-4; остальное — остаточный риск; TC-CORE-01 и TC-CORE-34 приведены в соответствие; PG-1…PG-6 и O-14 сохранены | SENTINEL_CORE_DESIGN.md §4.3; SENTINEL_CORE_TEST_PLAN.md | TC-CORE-01, TC-CORE-34 |
| Находка команды | Подавление пары могло исключить из fallback правило с `on_indeterminate = BLOCK` и тем ослабить результат (модель: 8 случаев из 20 000) | Fallback учитывает и правила, которые стали бы победителями без подавления (SENTINEL_CORE_DESIGN.md §6.3, шаг 8) | SENTINEL_CORE_DESIGN.md §6.3 | TC-CORE-43 |
| Находка команды | Свойство «недействительное исключение строже действительного» в общем случае неверно: исключение меняет интерпретацию неизвестного (модель: 389 случаев из 10 000, в основном правила с отрицанием атома содержимого) | Гарантия сформулирована точно: недействительное исключение равносильно отсутствию записи | SENTINEL_CORE_DESIGN.md §6.8 | G-24 |

F-01…F-06, R4-01…14, AR5-01…05, AR6-01 сохранены. D-01…D-17 и принятые ADR не изменены; FAILURE_MODE_DESIGN.md в rev8 не изменялся. D-18…D-21 — Proposed. Продуктовые тесты — Not Run; механизмы — NOT VERIFIED; алгоритм — Model Verified.

### CHG-0005, ревизия 7 — интеграционный кандидат (2026-11-12)

Основание: независимый аудит rev6 — Conditional Approval; «CHG-0005 rev6: Final Normative Integration». Изменения принятых документов внесены только в локальный интеграционный кандидат поверх rev6, не применены к утверждённому Git baseline и вступают в силу после Owner Approval.

| ID | Что было | Что сделано | Где | Тесты |
| --- | --- | --- | --- | --- |
| CHG5-AR6-01 | INV-C11 утверждал, что `required_action` при `OUTCOME_INDEPENDENT_OF_UNKNOWN` берётся только из `IncompleteInspectionAction` — противоречие с INV-C13 | Запись задаёт действие только исхода «победителя нет»; итоговое — самое строгое среди возможных (SENTINEL_CORE_DESIGN.md §6.3, шаг 7). Формулировки выровнены в FAILURE_MODE_DESIGN.md §3, DECISION_LOG.md (D-19), TC-CORE-33 | SENTINEL_CORE_DESIGN.md §10.3; FAILURE_MODE_DESIGN.md §3; DECISION_LOG.md §4.1; SENTINEL_CORE_TEST_PLAN.md | TC-CORE-37 (контрпример); G-22 |
| РВ-1 | ADR-0018 (раздел 5) и строка риска T-14 в терминах варианта A | Применён проект diff rev6; добавлено «проверенное частичное извлечение»: при отказе рабочего процесса без него — только `NOT_INSPECTED` | ADR-0018 (раздел 5); SENTINEL_CORE_DESIGN.md §4.1, §10.2 | TC-CORE-40; TC-D14-01 |
| РВ-2 (a) | Фраза ограничения D-14 противоречила D-19 | Строка D-14 не изменена; в D-19 — оговорка о приоритете новой семантики над этой фразой | DECISION_LOG.md §4.1 | — |
| РВ-3, РВ-4 | FAILURE_MODE_DESIGN.md §2.2, FI-D05-08, TC-D12-01, TC-D14-01, TC-D14-02, TC-SEC-04 | Применён проект diff rev6 | FAILURE_MODE_DESIGN.md, TEST_STRATEGY.md | Указанные TC |
| РВ-5 | THREAT_MODEL.md T-14 | Применён проект diff rev6 | THREAT_MODEL.md | — |
| OBS-02 | — | `input_digest` не свидетельствует о добросовестности рабочего процесса; результат `UNTRUSTED`; R-25 | SENTINEL_CORE_DESIGN.md §3.2, §13.1; ADR-0018 §5 | TC-CORE-41 |
| OBS-03 | — | Код 0 в режиме `status` — успешное выполнение, а не ALLOW и не полное покрытие | SENTINEL_SCAN_CLI_DESIGN.md §4.3 | TC-SCAN-14 |
| OBS-04 | — | Требования TA-1…TA-7 к отзыву и ротации `ReleaseTrustAnchors` до реализации `EnforcementReady`; вопрос O-16 | SENTINEL_CORE_DESIGN.md §6.7.1; SENTINEL_SCAN_CLI_DESIGN.md §2 | TC-CORE-42 |
| SEC-01…03 | — | Сверены с DPR-01, DPR-02, DPR-05/06; самостоятельных правил не введено | SENTINEL_CORE_DESIGN.md §13.2 | — |
| Согласованность | Порядок 12.2 перед 12.1; таблица рисков внутри 12.1 | Порядок исправлен; таблица соответствия решений и документов (SENTINEL_CORE_DESIGN.md §12.3); перечень Proposed и NOT VERIFIED (SENTINEL_CORE_DESIGN.md §13.3); O-15 — явная зависимость от TC-CORE-39 и риск ложного fallback BLOCK; Model Verified вместо «проверено» | SENTINEL_CORE_DESIGN.md §12, §13; SENTINEL_CORE_TEST_PLAN.md §5 | — |

F-01…F-06 и R4-01…14 сохранены. D-01…D-17 не изменены (оговорка РВ-2 — в записи D-19). ADR-0016 и ADR-0019 не изменены. D-18…D-21 и изменения РВ-1…РВ-5 — Pending Owner Approval. Продуктовые тесты — Not Run; механизмы — NOT VERIFIED; алгоритм — Model Verified.

### CHG-0005, ревизия 6 — узкие исправления CHG5-AR5-01…05 (2026-11-12)

Основание: независимое ревью rev5 — Changes Requested (полный патч rev5 `fc0972f1…`, дерево `721a1368…`). Новый полный цикл не запускался.

| ID | Что было | Что сделано | Где | Тесты |
| --- | --- | --- | --- | --- |
| CHG5-AR5-01 | Противоречия проекта D-19 с принятыми ADR-0018 (раздел 5), FI-D05-08, TC-D14-01, TC-SEC-04 не выявлены | Выявлены и перечислены 11 расхождений (AC-01…AC-11), в том числе ограничение D-14 и FAILURE_MODE_DESIGN.md §2.2. Принятые документы не изменены; проект изменений — отдельный diff вне патча; запрос разрешения владельцу | SENTINEL_CORE_DESIGN.md §12.2; DECISION_LOG.md §4.1 (D-19); PROJECT_STATE.md | — (критерии в проекте diff) |
| CHG5-AR5-02 | `required_action` при ALLOW в нескольких мирах неоднозначен | Сравнены варианты; контракт: самое строгое возможное действие, `action_certainty`, `action_sources`, семантика аудита и ALERT; INV-C13 | SENTINEL_CORE_DESIGN.md §5, §6.3 шаг 7, §6.4, §10.3, §12.1; SENTINEL_SCAN_CLI_DESIGN.md §4.2 | TC-CORE-37; G-15, G-19 |
| CHG5-AR5-03 | Поведение `file.class` при AMBIGUOUS не определено | Атомы `file.class` неизвестны; класс — измерение мира; непересечение по `file.class` доказывается только по константам бандла; `file.class` входит в условие (a) конфликта; INV-C14 | SENTINEL_CORE_DESIGN.md §4.3, §6.3, §6.7, §10.3 | TC-CORE-38; G-20 |
| CHG5-AR5-04 | Утверждения о точности не ограничены моделью независимых счётчиков | Граница утверждений зафиксирована; ложный fallback BLOCK при связанных детекторах показан (G-21); алгоритм не изменён; вопрос O-15; корпусный тест | SENTINEL_CORE_DESIGN.md §6.3 шаг 6, §10.3 (INV-C2), §12, §13 | TC-CORE-39; G-21 |
| CHG5-AR5-05 | Формулировки «нормативный», «целевое решение», «принятая редакция», «отклонённый вариант» | Заменены на Target Design (Proposed) до Owner Approval | SENTINEL_CORE_DESIGN.md, SENTINEL_SCAN_CLI_DESIGN.md, SENTINEL_CORE_TEST_PLAN.md, FAILURE_MODE_DESIGN.md §2.1, §3, §6.1, DECISION_LOG.md §4.1 | — |
| Находка команды | При AMBIGUOUS правило ALERT, зависящее только от `file.class`, выше правила BLOCK давало разрешающий fallback против достоверного BLOCK без конфликта (найдено моделью после ввода измерения класса) | `file.class` включён в условие (a) | SENTINEL_CORE_DESIGN.md §4.3, §6.7 | TC-CORE-38; G-20 |
| OBS-02…OBS-04 | Не зафиксированы | Текст замечаний команде не передан; формулировки не восстанавливаются по догадке | SENTINEL_CORE_DESIGN.md §13; PROJECT_STATE.md | — |

F-01…F-03 и R4-01…14 сохранены. D-01…D-17, ADR и CLAUDE.md не изменены. D-18…D-21 — Proposed. Продуктовые тесты — Not Run; механизмы — NOT VERIFIED.

### CHG-0005, ревизия 5 — целевая интеграция решений и уточнений ревью DPR-01…07 (2026-11-12)

Основание: независимое ревью DECISION_APPROVAL_PACKAGE_CHG-0005.md («CHG-0005 rev5: Conditional Decision Integration»). Идентификаторы DPR-01…07 — номера обязательных уточнений этого ревью в порядке их перечисления.

| ID | Что было | Что сделано | Где | Тесты |
| --- | --- | --- | --- | --- |
| D-18 (O-01 B) | Неполнота → `NOT_INSPECTED`, найденное отбрасывается | `MATCH` при неполном покрытии; монотонные атомы; атом `inspection.incomplete` вместо `inspection.status = NOT_INSPECTED` | SENTINEL_CORE_DESIGN.md §4.1, §5, §6.1; FAILURE_MODE_DESIGN.md §2.1 | TC-CORE-25, G-04, G-05, G-11 |
| D-19 (O-02 B) | Без явного условия → INDETERMINATE | INDETERMINATE, только если неизвестное может изменить вердикт; точная оценка по мирам; fallback среди возможных победителей | SENTINEL_CORE_DESIGN.md §6.3, §6.4; FAILURE_MODE_DESIGN.md §3, §6.1; TEST_STRATEGY.md | TC-CORE-12, -17, -32; G-01, G-02, G-09, G-15, G-17 |
| D-20 (O-11 C) | Атом `uninspected_only` в правиле | Отдельная запись `PartInspectionExclusion`; атом удалён | SENTINEL_CORE_DESIGN.md §4.2, §6.7 | TC-CORE-28; G-06, G-07, G-12, G-16 |
| D-21 (S-01 B) | Предложение | Target Design (Proposed); режим разработчика только параметром | SENTINEL_SCAN_CLI_DESIGN.md §7 | TC-SCAN-10, -13 |
| DPR-01 | Разрешающий fallback мог перекрыть достоверный BLOCK | Проверка «Конфликт приоритетов» при компиляции и загрузке; запись `PrecedenceConflictApproval`; INV-C12 | SENTINEL_CORE_DESIGN.md §6.3, §6.7, §10.3; FAILURE_MODE_DESIGN.md §3 | TC-CORE-12, -35; G-10, G-13, G-14, G-18 |
| DPR-02 | Источник MONITOR для `OUTCOME_INDEPENDENT_OF_UNKNOWN` не определён | Обязательная подписанная запись `IncompleteInspectionAction`; сигнал неполной инспекции отделён от потери возможности канала; INV-C11 | SENTINEL_CORE_DESIGN.md §6.1, §6.3, §6.7; FAILURE_MODE_DESIGN.md §3, §6.1 | TC-CORE-30, -33 |
| DPR-03 | Приоритеты G-11 не заданы | R11 — 20, R12 — 10; обратный порядок — конфликт; INV-C10; property-тесты монотонности | SENTINEL_CORE_TEST_PLAN.md §5; SENTINEL_CORE_DESIGN.md §10.3 | TC-CORE-25, G-11 |
| DPR-04 | Утверждение исключения через структуру ADR-0019 | Отдельная сущность `PartInspectionExclusion`; переиспользуются только требования к согласованию | SENTINEL_CORE_DESIGN.md §4.2 | TC-CORE-28 |
| DPR-05 | PG-2: поиск сигнатур в привилегированном сервисе | Запрещён; проба — только в рабочем процессе, в v0.1 выключена; O-14; замер TC-CORE-36; идентификация всех полиглотов не обещается | SENTINEL_CORE_DESIGN.md §4.3, §12 | TC-CORE-34, -36 |
| DPR-06 | Подозрительная сигнатура → неразобранная часть | `classification = AMBIGUOUS` отдельно от доказанных частей | SENTINEL_CORE_DESIGN.md §4.2, §4.3, §5 | TC-CORE-34; G-16 |
| DPR-07 | OBS-02…04 не зафиксированы | Текст замечаний команде не передан; зафиксировано как ожидание в PROJECT_STATE.md | PROJECT_STATE.md | — |
| Находка команды | Приближение «U выше W → INDETERMINATE» (Decision Approval Package, У-4) | Даёт ложный BLOCK при коррелированных порогах (выявлено моделью); заменено точной оценкой, приближение оставлено как шаг 5 | SENTINEL_CORE_DESIGN.md §6.3, §6.4 | TC-CORE-32; G-15, G-17 |
| Находка команды | Условие конфликта (a) «оба `on_indeterminate` разрешающие» пропускало разрешающий fallback против достоверного BLOCK через третье правило (контрпример построен вручную и подтверждён моделью) | Условие (a): достаточно разрешающего `on_indeterminate` у правила MONITOR или ALERT, зависящего от содержимого | SENTINEL_CORE_DESIGN.md §6.7 | TC-CORE-35; G-14, G-18 |
| Находка команды | Атом `uninspected_only` несовместим с O-02 B | Заменён исключением анализа частей | SENTINEL_CORE_DESIGN.md §4.2, §6.4 | G-12 |
| Попутно | Строка риска «в CLI изоляции нет» устарела после ревизии 2 | Исправлена | SENTINEL_CORE_DESIGN.md §12 | — |

D-01…D-17, ADR-0016, ADR-0018, ADR-0019, CLAUDE.md и DATA_MODEL.md не изменены. Решения D-18…D-21 — проекты (Proposed), статус Accepted не присвоен. Продуктовые тесты — Not Run; механизмы — NOT VERIFIED.

### CHG-0005, ревизия 4 — исправления по внутреннему ревью ревизии 3 (2026-11-12)

| ID | Исправление | Где |
| --- | --- | --- |
| R4-01 (HIGH) | Хеш не был связан с байтами, разобранными рабочим процессом. `TRUSTED_SNAPSHOT` — только при извлечении в том же процессе; извлечение в рабочем процессе — `UNTRUSTED` в v0.1; рабочий процесс возвращает `input_digest`, несовпадение с `full` → `CONTENT_CHANGED` | SENTINEL_CORE_DESIGN.md §3.2, §5; INV-C8; TC-CORE-26 |
| R4-02 (HIGH) | INV-H2: ключ кэша — полный ключ FAILURE_MODE_DESIGN.md §12; в кэш — только `TRUSTED_SNAPSHOT`, `decision_origin = PDP`, ALLOW или BLOCK, `MATCH` или `NO_MATCH` при полном покрытии; INDETERMINATE, `NOT_INSPECTED` и fallback не кэшируются | SENTINEL_CORE_DESIGN.md §3.2; TC-CORE-26 |
| R4-03 (HIGH) | Шаг 4 проверял наличие атома, а не его роль; значение атомов по содержимому при `NOT_INSPECTED` не было определено. Атомы по содержимому — UNKNOWN, логика Клини; атом `NOT_INSPECTED` — только конъюнктом верхнего уровня, под OR и NOT бандл отклоняется; понятие «допустимое основание» | SENTINEL_CORE_DESIGN.md §6.1, §6.3; TC-CORE-12, TC-CORE-27; G-09 |
| R4-04 (HIGH) | Подлинность манифеста возможностей не проверялась. `CapabilityManifest::verify` по `ReleaseTrustAnchors`, отдельным от якорей политики; `load_enforcement_ready` принимает только `VerifiedManifest`; CLI: `--manifest-trust-anchor` | SENTINEL_CORE_DESIGN.md §2.3, §6.7; O-05; SENTINEL_SCAN_CLI_DESIGN.md §2; TC-CORE-27, TC-SCAN-11 |
| R4-05 (HIGH) | CLI мог вернуть код 0 без вердикта. `EngineFault` → `error = ENGINE_FAULT`, `decision = null`, код 12; CLI fallback не применяет, `decision_origin` в выводе только `PDP`; код 0 только при записи без ошибки по каждому входу | SENTINEL_SCAN_CLI_DESIGN.md §4.2, §4.2.1, §4.3; SENTINEL_CORE_DESIGN.md §2.3, §10.2; TC-SCAN-03 |
| R4-06 (MEDIUM) | Поиск конца центрального каталога ZIP в сервисе противоречил ADR-0018 (раздел 1). Только сигнатура в начале файла; ZIP с префиксом и полиглоты — `UNSUPPORTED_FORMAT` или `PARTIAL`, никогда `NO_MATCH` | SENTINEL_CORE_DESIGN.md §4.3; TC-CORE-01 |
| R4-07 (MEDIUM) | Выбор победителя мог нарушить INV-C1: при `NOT_INSPECTED` победитель выбирается только среди допустимых оснований | SENTINEL_CORE_DESIGN.md §6.3; INV-C1; TC-CORE-12; G-10 |
| R4-08 (LOW) | Комментарий к `required_action`: отсутствует только при `NO_RULE_MATCHED`; при INDETERMINATE — по шагу 8 | SENTINEL_CORE_DESIGN.md §5; SENTINEL_SCAN_CLI_DESIGN.md §4.2 |
| R4-09 (MEDIUM) | TC-CORE-17 проверял вариант B и несуществующее состояние `HARD_EXPIRED`. Ожидания приведены к варианту A и состояниям ABSENT, INVALID, EXPIRED | SENTINEL_CORE_TEST_PLAN.md: TC-CORE-17 |
| R4-10 (MEDIUM) | Смешение POLICY_INVALID и POLICY_ABSENT. INVALID — повреждён сохранённый действующий бандл без резервной копии; новый недостоверный бандл состояние не меняет | SENTINEL_CORE_DESIGN.md §6.3, §10.2; TC-CORE-17, TC-CORE-18 |
| R4-11 (MEDIUM) | TC-CORE-27 допускал загрузку бандла с `uninspected_only` в `InspectionOnly`. Нарушения состава отклоняют бандл в обоих режимах | SENTINEL_CORE_DESIGN.md §6.7; TC-CORE-27 |
| R4-12 (MEDIUM) | Вариант C был неполон: не определены шаги 3 и 4. Добавлены вычисление атома и допустимое основание с `uninspected_only` | SENTINEL_CORE_DESIGN.md §6.4; TC-CORE-28 |
| R4-13 (MEDIUM) | Цикл в графе crates (`sentinel-core` ↔ базовые crates). Фасад зависит от базовых crates, не наоборот; правило ацикличности; диаграмма дополнена `sentinel-worker-host` | SENTINEL_CORE_DESIGN.md §2.2; TC-CORE-22 |
| R4-14 (LOW) | Ссылки: TC-CORE-16 — шаг 6; SENTINEL_CORE_DESIGN.md §5 — шаг 7; матрица трассировки — функции загрузки вместо `verify_and_load` | SENTINEL_CORE_TEST_PLAN.md |
| Дополнительно | Заголовок журнала (дата), строка проверок, диапазон открытых вопросов O-01…O-13; golden-ожидания G-09 и G-10 | CHANGELOG.md, SENTINEL_CORE_TEST_PLAN.md §5 |

D-01…D-17 и принятые ADR не изменены. O-01, O-02, O-11 и S-01 остаются Pending. UX Direction Proposal (CHG-0006) в CHG-0005 не входит.

### CHG-0005, ревизия 2 — исправления по архитектурному ревью (2026-11-12)

| ID | Исправление | Где |
| --- | --- | --- |
| CHG5-AR-01 (HIGH) | `ContentDigest`: полный хеш только при полном последовательном чтении; доверие `TRUSTED_SNAPSHOT` только при заявленном хостом неизменном источнике и неизменной длине; хеш прочитанных диапазонов не используется для кэша и TOCTOU; изменение длины → `CONTENT_CHANGED` | SENTINEL_CORE_DESIGN.md §3, §3.2; INV-C8; TC-CORE-26 |
| CHG5-AR-02 (HIGH) | Снята гарантия «всегда возвращает Decision». `evaluate` возвращает `Result` с `EngineFault`; таблица отказов процесса, IPC, `panic=abort`, памяти, стека и зависания с ответственностью хоста; обязательство хоста INV-H1 | SENTINEL_CORE_DESIGN.md §2.3, §10.2; TC-CORE-13, TC-CORE-29 |
| CHG5-AR-03 (HIGH) | Повторная проверка бандла по ADR-0016 и ADR-0019: платформа, BLOCK_CAPABLE с `verified_by` для платформы, канала и операции, записи исключений, `on_indeterminate = BLOCK`, UNSUPPORTED; отметка `capability_check = SKIPPED` без манифеста; потеря возможности во время работы — у PEP | SENTINEL_CORE_DESIGN.md §6.1, §6.7; INV-C9; TC-CORE-27, TC-SCAN-11 |
| CHG5-AR-04 (HIGH) | Разделены `text_coverage` и `object_coverage`; `NO_MATCH` только при полном тексте и всех разобранных частях; предложен безопасный вариант C для O-11, вариант B снят | SENTINEL_CORE_DESIGN.md §4.2, §5; INV-C4; TC-CORE-04, TC-CORE-28 |
| CHG5-AR-05 (HIGH) | CLI: по умолчанию изолированный рабочий процесс; без него OOXML и PDF не разбираются (`ISOLATION_REQUIRED`); режим разработчика — только явно, с предупреждением и отказом при правах администратора; механизм песочницы не выбран | SENTINEL_SCAN_CLI_DESIGN.md §1, §7; TC-SCAN-10 |
| CHG5-AR-06 (MEDIUM) | Узкий фасад `sentinel-core-extract-api` для рабочего процесса; проверка графа каждого артефакта по списку разрешённых зависимостей | SENTINEL_CORE_DESIGN.md §2.2; TC-CORE-22 |
| CHG5-AR-07 (MEDIUM) | CLI: по умолчанию непрозрачный `input_id`, без путей и SHA-256; диагностическое раскрытие только явным параметром с предупреждением | SENTINEL_SCAN_CLI_DESIGN.md §4.2, §5; TC-SCAN-05 |
| CHG5-AR-08 (MEDIUM) | `required_action = NONE` удалён; отсутствие правила — `decision_basis = NO_RULE_MATCHED` без `required_action`, зависимость DATA_MODEL.md; приоритет кодов выхода CLI | SENTINEL_CORE_DESIGN.md §5, §6.3; SENTINEL_SCAN_CLI_DESIGN.md §4.3; TC-CORE-30, TC-SCAN-03 |
| Дополнительно | Определение TXT и CSV без сигнатуры, полиглоты; O-12 | SENTINEL_CORE_DESIGN.md §4.3; TC-CORE-01 |

O-01, O-02, O-11 и S-01 остаются Pending.

### CHG-0005, ревизия 3 — целевые исправления по ревью ревизии 2 (2026-11-12)

| ID | Исправление | Где |
| --- | --- | --- |
| CHG5-AR2-01 (HIGH) | Доверие к хешу определяется происхождением снимка, а не заявлением хоста: в v0.1 доверенное происхождение одно — `CORE_OWNED_BUFFER`; дескрипторы, запечатанная память и теневая копия — `UNTRUSTED` до SPIKE (O-13); обязательство хоста INV-H2; изменение содержимого при той же длине; нет гарантии L3 | SENTINEL_CORE_DESIGN.md §3, §3.2; TC-CORE-26 |
| CHG5-AR2-02 (HIGH) | Режимы загрузки `EnforcementReady` и `InspectionOnly` — разные типы результата; `SKIPPED` только в `InspectionOnly`; PEP не принимает `InspectionOnly`; `policy verify` выводит раздельно `signature` и `enforceability` | SENTINEL_CORE_DESIGN.md §2.3, §6.7; SENTINEL_SCAN_CLI_DESIGN.md §2, §3; TC-CORE-27, TC-CORE-31, TC-SCAN-11, TC-SCAN-12 |
| CHG5-AR2-03 (HIGH) | Условие `uninspected_parts ⊆ {IMAGE}` заменено атомом `uninspected_only(kinds)`: полное перечисление структуры, непустой перечень, без `UNKNOWN`, правило с ограниченной областью и утверждением; при неполном перечислении — UNKNOWN и fallback; O-11 Pending | SENTINEL_CORE_DESIGN.md §4.2, §6.7; TC-CORE-28 |
| CHG5-AR2-04 (HIGH) | Разделены вердикт PDP, отсутствие вердикта, fallback хоста и автономный fallback PEP; поля `decision_origin` и `inspection_origin`; INDETERMINATE за непрочитавший PDP не синтезируется; INV-H1 уточнён; ADR-0016 не изменён | SENTINEL_CORE_DESIGN.md §2.3, §10.2; TC-CORE-29 |
| CHG5-AR2-05 (MEDIUM) | Снято противоречие о размещении этапов в CLI; три последовательности; компоненты `sentinel-worker-host` (вне ядра) и `sentinel-extract-worker` | SENTINEL_CORE_DESIGN.md §2.2, §4; SENTINEL_SCAN_CLI_DESIGN.md §1, §7.1; TC-SCAN-08, TC-SCAN-10 |
| CHG5-AR2-06 (MEDIUM) | Нормативна семантика FAILURE_MODE_DESIGN.md §6.1 (варианты A); варианты B и C — предложения; golden-ожидания G-01…G-08 по каждому варианту | SENTINEL_CORE_DESIGN.md §6.3, §6.4; SENTINEL_CORE_TEST_PLAN.md §5; TC-CORE-11, -12, -25 |

O-01, O-02, O-11 и S-01 остаются Pending Approval. UX Direction Proposal (CHG-0006) в CHG-0005 не входит.

## CHG-0004 — Решения D-12…D-17 по конфликтам C-01…C-06; ADR-0018, ADR-0019

| Поле | Значение |
| --- | --- |
| Change ID | CHG-0004 |
| Дата и версия | 2026-11-11; ревизия 2 — 2026-11-12; версия продукта не присвоена (кода нет). База — дерево `1defee0279b620812848171ca279e55b722d7125` коммита `87c610be` (Draft PR #1) |
| Инициатор | Владелец продукта: C-01…C-06 Architecture Approval от 2026-11-11 (Approved для подготовки CHG-0004); разрешение на локальную документационную реализацию CHG-0004 от 2026-11-11 с условиями ревьюера 1–6 |
| Что было | C-01…C-06 — Pending Approval (DECISION_PROPOSAL_C01_C06.md, версия 2); в документах ссылки «C-0N (Pending)»; ADR для C-03 и C-06 нет; 88 функций; риски R-01…R-24; 122 тест-кейса |
| Что изменено | Добавлены ADR-0018 (изоляция разбора недоверенных форматов на агенте; Accepted in principle) и ADR-0019 (проверка возможностей при компиляции и контролируемые исключения; Accepted). DECISION_LOG.md: D-12…D-17 со степенью окончательности, ограничения, проверки, новый Pending-вопрос о JUSTIFY и QUARANTINE, трассировка, записи утверждений (разделы 2, 3, 4, 6, 7). FEATURE_REGISTRY.md: новая DLP-ENDPOINT-021 (D-14, Proposed); уточнены DLP-POLICY-012, -014, -015, DLP-ENDPOINT-015, DLP-INCIDENT-003 и примечания. SYSTEM_ARCHITECTURE.md: реестр ADR (0018, 0019; пометки 0003, 0007, 0014, 0015), новый риск R-25, уточнён R-11. TEST_STRATEGY.md: TC-D12-01…TC-D17-06 (19); уточнены TC-D08-01 и формулировки TC-SEC-02, TC-SEC-04 (результат `NOT_INSPECTED` до вердикта, условие ревьюера 4). THREAT_MODEL.md: T-05, T-14, T-32. CAPABILITY_MATRIX.md §4, §4.3, §4.4. ROADMAP.md: D-12…D-17, Network PoC вне MVP, G1, G2. FAILURE_MODE_DESIGN.md §2.1, §7.1, §15 (пункты 11–12 SPIKE-WIN-01), INV-E4 и FI-D05-08. DATA_PROTECTION_DESIGN.md §4. DECISION_PROPOSAL_C01_C06.md: статус «Решено», итоговая таблица. DOCUMENTATION_RECONCILIATION.md: ревизия 3, пометки решений. PROJECT_STATE.md, CHANGELOG.md |
| Причина | Оформление решений владельца по C-01…C-06 с точной степенью окончательности и устранение ссылок на Pending-статусы |
| Затронутые компоненты | Только документация: Endpoint Agent, Detection Engine, Policy Engine (компилятор), Management Server (поставка), Incident Management (хранилище evidence), Network Gateway (фаза) |
| Связанные требования | CLAUDE.md §4–§11; D-05, D-07, D-08; D-12…D-17; DLP-ENDPOINT-021, DLP-POLICY-012, DLP-POLICY-014, DLP-POLICY-015, DLP-ENDPOINT-015, DLP-INCIDENT-003 |
| Изменения API / БД | Нет. ADR-0019 вводит на уровне дизайна структуру `CapabilityException` и поля исключения в событии; DATA_MODEL.md не менялся (вне согласованного состава) — обновляется при переходе DLP-POLICY-015 в Designed |
| Риски | Новый риск R-25 (нарушение границы песочницы разбора); R-11 дополнен задержкой IPC. D-13 — предварительно, D-14 и D-15 — в принципе: механизмы NOT VERIFIED. ADR-0007 и ADR-0014 остаются Proposed и частично расходятся с D-16 и D-13 (зафиксировано в реестре ADR) |
| Совместимость | D-01…D-11, ADR-0001…0006, ADR-0016, ADR-0017 не изменены. Категории сверки RECON-0001 (71/23/10/2) не изменены |
| Миграции | Нет |
| Результаты проверок | Выполнены локально 2026-11-11, подробности — раздел «CHG-0004: проверки» ниже |
| Откат | До фиксации — не применять патч. После фиксации — `git revert` |
| Статус | Ревизия 2 по CHG-0004 Architecture Review (Changes Requested, narrow corrections) от 2026-11-12. Подготовлено локально как патч поверх дерева PR #1. В Git не зафиксировано; PR не создавался; PR #1 не изменялся. Продуктовые механизмы — NOT VERIFIED. **Обновление 2026-11-12:** Release Candidate Accepted; опубликовано и влито в `develop` (раздел «CHG-0004: публикация и merge») |

### CHG-0004: публикация и merge (2026-11-12)

| Шаг | Дата | Результат |
| --- | --- | --- |
| Release Candidate | 2026-11-12 | Accepted. Патч SHA-256 `2ff1670a9e1b249a877c1352ec765afed3c43a56883f40bd39c71f542b71db17`; 44 + 13 проверок PASS |
| Preflight после merge PR #1 | 2026-11-12 | `develop@322dade`, дерево `1defee0`; `git apply --check --whitespace=error-all` без замечаний; итоговое дерево `17cbb22` |
| Коммит и push по D-10 | 2026-11-12 | Коммит `448134651987511fc36621df82119562f7db83e5` (родитель `322dade`, дерево `17cbb22c8e7562a6e38cfa315435b1c28e40334e`) в `docs/chg-0004`; push без force |
| PR #2 | 2026-11-12 | Draft PR `docs/chg-0004` → `develop`, 15 файлов; независимое ревью — PASS |
| Merge | 2026-11-12 | Влит владельцем merge-коммитом `51422176cd3f56ecc62360f1549e402253a3bd07` (родители `322dade`, `448134`; дерево `17cbb22`). `main` не изменялся |

Хронология ревизий 1 и 2 ниже сохранена без изменений. Продуктовые механизмы — NOT VERIFIED.

### CHG-0004, ревизия 2 — узкие исправления по архитектурному ревью (2026-11-12)

| ID | Что изменено |
| --- | --- |
| CHG4-AR-01 (HIGH) | ADR-0019 версия 2. Раздел 3 — поля `requested_action` и `capability_exception_id`, зависимость DATA_MODEL.md до реализации DLP-POLICY-015. Раздел 4 — значения полей и критерии исполнения по `required_action`, утрата BLOCK_CAPABLE независимо от исключений, `ENFORCEMENT_FAILED` только при `required_action = BLOCK`. Раздел 5 — детерминированные сценарии 1–9. FAILURE_MODE_DESIGN.md: структура `EnforcementResult`, INV-E3, INV-E4, новый раздел 2.4a. CAPABILITY_MATRIX.md §4.4. TC-D17-02, TC-D17-04 изменены; добавлены TC-D17-05, TC-D17-06 |
| CHG4-AR-02 (MEDIUM) | ROADMAP.md G2: TC-D14-01…06, включая TC-D14-05 с проверкой утверждённого бюджета синхронного пути; TC-D16-02 явно отнесён к пилоту. Критерий TC-D14-05 дополнен |
| CHG4-AR-03 (LOW) | R-25: остаточный риск не снижается до испытаний реального механизма изоляции |
| Решения по Pending | JUSTIFY и QUARANTINE — Pending; до решения неподтверждённое исполнение → отказ публикации без понижения (ADR-0019, CAPABILITY_MATRIX.md §4.3, DECISION_LOG.md §4). DATA_MODEL.md не изменяется |

Не изменены: D-01…D-17 (строки решений и ограничений), ADR-0001…0006, ADR-0016, ADR-0017, ADR-0018. Затронутые файлы ревизии 2 (10): ADR-0019, FAILURE_MODE_DESIGN.md, CAPABILITY_MATRIX.md, TEST_STRATEGY.md, ROADMAP.md, SYSTEM_ARCHITECTURE.md, FEATURE_REGISTRY.md, DECISION_LOG.md, CHANGELOG.md, PROJECT_STATE.md.

### CHG-0004: проверки

Выполнены локально 2026-11-11 скриптом документальных проверок (расширенная версия проверок CHG-0003).

**PASS:**

- изменены только `docs/**/*.md`, и только 13 согласованных документов плюс два новых ADR (TC-D10-01);
- секретов, персональных данных, HTML и скриптов, встроенных инструкций и управляющих символов нет (TC-D10-02);
- относительные ссылки разрешаются;
- 89 ID функций уникальны и определены; T-01…T-33, R-01…R-25, D-01…D-17, Q-01…Q-09 и 141 тест-кейс определены (TC-D10-07);
- у каждого тест-кейса есть критерий; ссылки «ФАЙЛ.md §N» указывают на существующие заголовки;
- статусы ADR в файлах совпадают с реестром; у каждой угрозы есть проверка; таблицы согласованы;
- строки D-01…D-11 и ограничения D-05…D-10 побайтно не изменены; Q-01…Q-09 не изменены и открыты;
- файлы ADR-0001…0006, 0016, 0017 не изменены; ADR-0007 и ADR-0014 — Proposed, их файлы не созданы;
- число упоминаний NOT VERIFIED ни в одном изменённом файле не уменьшилось;
- каждый новый тест-кейс TC-D12…D17 упомянут вне TEST_STRATEGY.md;
- TC-D13-01 и D-13 не связаны с R-07; `FULL_EVIDENCE` нигде не заявлен включённым в MVP; абсолютного утверждения о безопасности памяти Rust без оговорок об `unsafe` и FFI нет;
- RECON-0001: 106 функций FID сопоставлены, категории 71/23/10/2 не изменились;
- патч применяется к дереву `1defee0` без предупреждений `git apply`.

**NOT RUN:** продуктовые тесты — кода нет.

**Независимое ревью (2026-11-11):** вердикт «Ready with fixes», Critical — нет. Исправлены Important I-1…I-7: пункты SPIKE-WIN-01 для D-14 и D-15; цепочка `NOT_INSPECTED` → вердикт → действие в TC-D12-01, TC-SEC-02, TC-SEC-04, FI-D05-08; в сервисе только сигнатура, подтип контейнера — в рабочем процессе; уточнён запрет доступа к реестру и дочерних процессов; JUSTIFY и QUARANTINE вынесены в открытый вопрос; уточнены строки таблицы ADR-0019; golden-тест без разбора в процессе сервиса. Исправлены Minor M-1…M-10, M-12, M-13. M-11 (новые критерии G2) вынесен на подтверждение владельцу; M-14 (DATA_MODEL.md вне состава) — зарегистрирован как открытый пункт.

## CHG-0003 — Решения D-05…D-11, сверка с веткой feature/initial-documentation, дизайн отказоустойчивости, конвейера событий и защиты данных

| Поле | Значение |
| --- | --- |
| Change ID | CHG-0003 |
| Дата и версия | 2026-11-10; ревизии 4 и 4.1 — 2026-11-11; версия продукта не присвоена (кода нет) |
| Инициатор | Владелец продукта — решения D-05…D-10, D-11, разрешение на локальную подготовку от 2026-11-10, решение по дизайну ревизии 4 от 2026-11-11 |
| Что было | CHG-0002: ADR-0001…0004, DECISION_LOG (D-01…D-04), REFERENCE_ANALYSIS, PROJECT_STATE |
| Что изменено | Добавлены: ADR-0005, ADR-0006, ADR-0016, ADR-0017; architecture/FAILURE_MODE_DESIGN.md, EVENT_PIPELINE_DESIGN.md, CAPABILITY_MATRIX.md, DOCUMENTATION_RECONCILIATION.md (RECON-0001, ревизия 2), DECISION_PROPOSAL_C01_C06.md; security/DATA_PROTECTION_DESIGN.md; SYSTEM_ARCHITECTURE.md, THREAT_MODEL.md, DATA_MODEL.md, FEATURE_REGISTRY.md, ROADMAP.md, TEST_STRATEGY.md. Изменены: ADR-0004 (версия 2, ограничения D-05), DECISION_LOG.md (D-05…D-11, ограничения, проверки, вопросы, трассировка), REFERENCE_ANALYSIS.md, PROJECT_STATE.md, CHANGELOG.md |
| Причина | Фиксация решений D-05…D-11 и сверка с параллельным комплектом документации до начала реализации |
| Затронутые компоненты | Архитектура всех компонентов (только документация) |
| Связанные требования | CLAUDE.md §4–§11, §13, §14; D-05…D-11 |
| Изменения API / БД | Нет. Логическая модель данных (DATA_MODEL.md) — Proposed; схема БД не создавалась |
| Риски | Новые риски R-15…R-24; угрозы T-12…T-33; в ревизии 4 R-24 переоценён (влияние «Высокое»: восстановление снимка ВМ с RAM). C-01…C-06 не решены; параметры D-05 и D-06 не измерены |
| Совместимость | Не применимо |
| Миграции | Нет |
| Результаты проверок | Ревизия 4.1, выполнены локально 2026-11-11 (подробности — разделы «ревизия 4.1» и «ревизия 4» ниже). **PASS:** изменены только `docs/**/*.md` (TC-D10-01); секреты, персональные данные, HTML и скрипты, встроенные инструкции и управляющие символы не найдены; внешние ссылки только из ожидаемого списка (TC-D10-02); относительные ссылки и пути разрешаются; 88 ID функций уникальны и определены; T-01…T-33 определены, у каждой угрозы есть карточка и проверка; R-01…R-24, D-01…D-11, Q-01…Q-09 и 122 тест-кейса определены (TC-D10-07); у каждого тест-кейса есть колонка «Критерий» с проверяемым условием, включая TC-SEC-01…12; 440 ссылок «ФАЙЛ.md §N» указывают на существующие заголовки (проверяется существование заголовка, а не смысл ссылки; смысл ссылок на изменённые разделы проверен вручную); у каждого критерия fault injection и сценария есть критерий; у каждой функции раздела 1 реестра есть критерий и тест; статусы ADR совпадают с реестром; 106 функций FID сопоставлены, 35 кандидатов совпадают с RECON-0001; во всех таблицах одинаковое число ячеек; патчи применяются без предупреждений `git apply`. **NOT RUN / NOT VERIFIED:** защита веток и права токена (TC-D10-05, TC-D10-06), `git push --dry-run` (TC-D10-04). Продуктовые тесты не применимы: кода нет |
| Откат | До фиксации — не применять патч. После фиксации — `git revert` |
| Статус | Design Accepted (решение владельца от 2026-11-11) с редакционной корректировкой ревизии 4.1. Подготовлено локально как патч; в Git не зафиксировано; Git-операции — отдельным согласованием по D-10. Продуктовые механизмы — NOT VERIFIED. **Обновление 2026-11-12:** CHG-0002 и CHG-0003 опубликованы одним коммитом `87c610be23e1df81f1f9f6119d2dba4344ebc44f` (PR #1, 2026-11-11) и влиты в `develop` владельцем 2026-11-12 merge-коммитом `322dade79ca872baae192a2c5a10807a1ce9dafc` (дерево `1defee0279b620812848171ca279e55b722d7125`) |

### CHG-0003, ревизия 4.1 — редакционная корректировка после Final Architecture Approval (2026-11-11)

Решение владельца: **Architecture Design Accepted**; SR4-01 и SR4-02 закрыты; решения 1–5 подтверждены.

Обязательная правка — утверждение «первая передача после восстановления снимка ВМ всегда получает `STALE_BATCH`» заменено тремя вариантами ответа и условием, при котором откат не обнаруживается:
- `batch_no = last_batch_no`, digest совпадает → первоначальный ACK, откат не обнаружен;
- `batch_no = last_batch_no`, digest различается → `BATCH_INTEGRITY_CONFLICT`;
- `batch_no < last_batch_no` → `STALE_BATCH`.

Необнаружение отката при полностью воспроизведённом состоянии и совпадающих данных передачи, а также при отсутствии обработанных передач после снимка, отнесено к R-24.

Затронуты: EVENT_PIPELINE_DESIGN.md §3.1, §4.8; FAILURE_MODE_DESIGN.md §4.7.1, §13 (FI-D05-23), §14 (R-24); TEST_STRATEGY.md (TC-D05-23, TC-D06-45), THREAT_MODEL.md (T-18, T-33), SYSTEM_ARCHITECTURE.md (R-24), CHANGELOG.md, DECISION_LOG.md, PROJECT_STATE.md. Архитектурные решения, алгоритмы, API и состав компонентов не менялись.

### CHG-0003, ревизия 4.1 — исправления по финальному ревью (Conditionally Approved, SR4-01, SR4-02)

| Замечание | Изменение |
| --- | --- |
| SR4-01 | Идентичность передачи `(endpoint_id, agent_epoch, batch_no, batch_digest)`. `batch_digest` — SHA-256 детерминированной каноникализации `SENTINEL-BATCH-V1`: элементы по возрастанию `seq`, `payload_hash` от полученных байтов тела. Сервер вычисляет digest сам и сверяет с заявленным: расхождение → `BATCH_MALFORMED`. `last_batch_digest` хранится в одной транзакции с `last_batch_no` и ответом. Тот же номер и тот же digest → исходный ACK; тот же номер и другой digest → `BATCH_INTEGRITY_CONFLICT` без изменения состояния и без старого ACK. Каждый ответ связан с `batch_no` и digest; агент не действует по чужому ответу и не удаляет элементы по нему. Определены действия агента при конфликте (остановка, флаг, процедура EVENT_PIPELINE_DESIGN.md §4.5.3), диапазон `batch_no` `1 … 2^63 − 1` и исчерпание счётчика. Новая угроза T-33. Тест-кейсы TC-D06-47…55, уточнён TC-D06-39 |
| SR4-02 | Модель разделена на шаги внутри логического пакета и транспорт полных передач. У каждой передачи свои `batch_no` и digest, обработка атомарна. Проверены порядок элементов внутри пакета, порядок доставки полных пакетов, атомарность, конкурентные и устаревшие передачи. Детерминированные, случайные и мутационные проверки повторены. Перечислено, что модель не представляет (EVENT_PIPELINE_DESIGN.md §4.10) |
| Решения владельца 1–5 | Отражены в EVENT_PIPELINE_DESIGN.md §3.1, §4.3, §4.4, §11; FAILURE_MODE_DESIGN.md §4.7.1; SYSTEM_ARCHITECTURE.md (R-24 — High); DECISION_LOG.md §7, §8 |

**Контрпримеры «до» и «после»:**
- CE-6 — откат агента и тот же `batch_no` с новым содержимым. Ревизия 4 возвращала чужой ACK, и агент удалял новые события. Ревизия 4.1 отвечает `BATCH_INTEGRITY_CONFLICT`, ничего не удаляется.
- CE-7 — запоздалый ACK. Ревизия 4 помечала записанное событие как окончательно отклонённое. Ревизия 4.1 такой ответ игнорирует.

**Результаты проверок ревизии 4.1:**

| Проверка | Passed | Failed | Not Run |
| --- | --- | --- | --- |
| Модель: шаги приёма внутри пакета | 30 | 0 | — |
| Модель: транспорт полных передач | 24 | 0 | — |
| Модель: мутации (каждая должна быть обнаружена) | 8 из 8 обнаружены; контрольный прогон чистый | 0 | — |
| Документальные проверки | 27 | 0 | — |
| Применимость патчей без предупреждений | 3 | 0 | — |
| PostgreSQL 16 | — | — | Не запускалось в 4.1: SQL аренды не менялся; хранение `last_batch_digest` — запись в той же строке, новых инструкций не добавлено |
| Защита веток, права токена, `git push --dry-run` (TC-D10-04…06) | — | — | Not Run: нет доступа и разрешения |
| Продуктовые тесты, включая TC-D06-47…55 | — | — | Not Run: кода нет |

**Наблюдения при проверке:**
- Первый прогон новой модели нашёл недочёт формулировки ревизии 4.1: связывание нужно не только для ACK, но и для `STALE_BATCH` и других отказов. Иначе ответ на старую задержанную передачу останавливал честного агента как «откат». Исправлено: каждый ответ связан с передачей (EVENT_PIPELINE_DESIGN.md §3.1 п. 6).
- Режим `TELEMETRY_INCOMPLETE` в случайных прогонах с полными передачами не достигается, поэтому проверен детерминированно. Отсечение по `batch_no` случайные прогоны не ловят, его ловит детерминированная проверка окончательности отказа.

Затронутые файлы ревизии 4.1 (11): EVENT_PIPELINE_DESIGN.md, FAILURE_MODE_DESIGN.md, DATA_MODEL.md, THREAT_MODEL.md, TEST_STRATEGY.md, DECISION_LOG.md, ROADMAP.md, SYSTEM_ARCHITECTURE.md, ADR-0006 (в области SR4-01), PROJECT_STATE.md, CHANGELOG.md. D-01…D-11 не изменены; C-01…C-06, `hard_expiry_at` и прочие Pending Approval не приняты; новые параметры не вводились, кроме фиксированного диапазона `batch_no`; порог `STALE_BATCH` и объём карантина остаются Proposed.

### CHG-0003, ревизия 4 — исправления по Security Architecture Review, раунд 3

Дизайн исправлений согласован владельцем 2026-11-11: Approved for Documentation with Constraints. Это не утверждение производственной реализации.

| Замечание | Изменение |
| --- | --- |
| SR3-01 | Без квоты принимается только голова очереди `lw + 1`; остальные события пропуска проходят окно и квоту. Дубликаты и конфликты проверяются первыми. Граница продвигается после каждого элемента пакета. Перед `COMMIT` проверяются инварианты, нарушение откатывает пакет. Инварианты INV-I1…I8 и контрпримеры CE-1…CE-4 |
| SR3-02 | Маркер `EVENTS_LOST` занимает слот первого гарантированно неотправленного события («надгробие»). Добавлены: состояния передачи событий у агента, `MARKER_RESERVE`, режим `TELEMETRY_INCOMPLETE`, отсечение устаревших передач `batch_no`, единое окно для событий и маркеров, `MAX_LOSS_SPAN`, правила для недостоверных маркеров, процедура восстановления при незакрываемом пропуске, лимиты ресурсов, повтор после `REJECTED_LIMIT`, продвижение границы через маркер за один шаг. `REJECTED_LIMIT` больше не записывается; поле `applied_ranges` удалено. Вариант `loss_seq` записан в ADR-0006 как рассмотренный |
| SR3-03 | Аренда вычисляется по `clock_timestamp()`. Захват, продление и завершение — короткие транзакции, обработка — вне транзакции, без удержания `FOR UPDATE`. Завершение после `lease_until` допустимо при актуальном токене. Определены потерянный ответ на `COMMIT` и сбой воркера. Fencing заявлен только для PostgreSQL; R-23 сохранён |
| SR3-04 | INV-T2 и INV-T3 стали условными; `T_boot_penalty` описан как эвристика. Добавлены сценарий восстановления снимка ВМ с RAM (FAILURE_MODE_DESIGN.md §4.7.1), таблица гарантий по видам отката, FI-D05-23 и FI-D05-24. VM Generation ID, TPM и vTPM — NOT VERIFIED. `hard_expiry_at` и офлайн break-glass по-прежнему Pending Approval |
| SR3-05 | Критерий crypto-shredding: отсутствие полной комбинации шифротекста, обёрнутого DEK и ключа во всех путях восстановления; общая формула `T_eff`; ограничения по резервным копиям, эскроу, legal hold и независимому аудиту сохранены |
| SR3-06 | Колонка критерия у TC-SEC-01…12; исправлены ссылки TC-D06-16, T-18, ADR-0006 и уровня Fault injection; уточнены краткие критерии TC-D05-11, TC-D07-02, TC-D10-02. В документальные проверки добавлены проверка колонки «Критерий» и проверка ссылок на разделы |

**Новые и изменённые тест-кейсы.**
- Добавлены TC-D05-23, TC-D05-24 и TC-D06-32…46.
- Изменены TC-D05-21, TC-D06-11, TC-D06-15…18, TC-D06-20, TC-D06-22, TC-D06-24, TC-D06-25, TC-D06-30, TC-D07-05, TC-D05-11, TC-D07-02, TC-D10-02 и TC-SEC-01…12.
- Всего 113 тест-кейсов.

**Независимое ревью черновика ревизии 4** (ревьюер без доступа к истории работы, только к патчу и требованиям). Вердикт — «с исправлениями». Учтено:

| Замечание ревью | Исправление |
| --- | --- |
| C-1: проверка маркера полуинтервалом допускала вложенный слот — утечка `pending_count`, остановка `lw` | Проверка по замкнутым интервалам против квитанций, восстановлений и деклараций; контрпример CE-5; TC-D06-43; мутация M6 |
| I-1: учёт квитанции головы очереди не определён | Явное правило шага 5: квитанция, `+1` и пересечение в том же шаге; `pending_count >= 0` в проверке перед `COMMIT` |
| I-2: после отката состояния агента новые события с выданными `seq` терялись как дубликаты | `STALE_BATCH` на текущую передачу — признак отката; агент не догоняет `batch_no`; `STATE_ROLLBACK_SUSPECTED`; допущение записано явно; TC-D05-23, FI-D05-23, TC-D06-45 исправлены и добавлены |
| I-3: протокол `batch_no` не доопределён | `batch_no` сохраняется до отправки; все `IN_FLIGHT` входят в следующую передачу; переходы по ответам на пакет целиком; TC-D06-46 |
| I-4: недопустимое событие в голове очереди останавливало доставку | Карантин: квитанция и `accepted(quarantined)`, запись `event_integrity_conflicts` (`SCHEMA_INVALID`); TC-D06-44 |
| I-5: исчез перевод «отравленных» заданий в `dead` | `CASE` на шаге 5, фильтр `attempts` в захвате, инструкция обслуживания; TC-D06-11 |
| I-6: критерий crypto-shredding не учитывал открытые копии и снятие legal hold | Область критерия; `T_obj = ∞` до подтверждения затирания; удаление обёртки `KEK_HOLD` при снятии удержания; TC-D07-05 |
| I-7: трассировка не обновлена | DECISION_LOG.md §6, ROADMAP.md (G2), сводная таблица THREAT_MODEL.md |
| Minor | Проверка формы маркера только в EVENT_PIPELINE_DESIGN.md §4.5.1; TC-D06-34 детерминирован; захват через `MATERIALIZED` CTE (не больше `$n` строк); повтор завершения при потерянном ответе; формулировки идемпотентности ACK, таблицы отказов EVENT_PIPELINE_DESIGN.md §4.8, режима `TELEMETRY_INCOMPLETE`, процедуры EVENT_PIPELINE_DESIGN.md §4.5.3, таблицы гарантий FAILURE_MODE_DESIGN.md §4.12, TC-SEC-01; `outbox` (`dead`) в EVENT_PIPELINE_DESIGN.md §9; проверка `MAX_LOSS_SPAN <= MAX_SEQ_AHEAD` при запуске |

**Повторная проверка исправлений тем же ревьюером.** C-1, I-1, I-3, I-5, I-6, I-7 закрыты; отказ по шагу 3 признан приемлемым. Найдены и исправлены четыре дефекта, внесённые исправлениями:

| Замечание | Исправление |
| --- | --- |
| N-1: процедура восстановления после отката удаляла новые события по итоговому `lw` | При `STATE_ROLLBACK_SUSPECTED` удаление по `lw` запрещено; все хранимые события переносятся в новую эпоху, сверка `event_id` с квитанциями и `events`; TC-D06-45 |
| N-2: `payload_hash` от канонического представления зависел от схемы, и карантин не срабатывал для агента новой версии | `payload_hash` — SHA-256 переданных байтов тела (EVENT_PIPELINE_DESIGN.md §2); TC-D06-44; срок хранения тел в карантине — класс P2, а не аудиторный |
| N-3: повтор той же передачи на транспортном уровне выглядел как откат | `batch_no = last_batch_no` → сохранённый ответ без повторной обработки; `STALE_BATCH` только при `batch_no < last_batch_no`; транспортные повторы отключены протоколом; TC-D06-39 |
| N-4: перевод в `dead` без проверки токена противоречил решению SR3-03 | Запас `$dead_grace` после `lease_until`; исключение описано в EVENT_PIPELINE_DESIGN.md §5.3; TC-D06-11 |

Итоговый вердикт повторной проверки — «Готово к финальному ревью: Да». Три мелких замечания учтены сразу:
- маркеры старой эпохи при переносе становятся сведениями о потере, а не маркерами новой эпохи;
- формат хранимых тел не меняется при обновлении агента (EVENT_PIPELINE_DESIGN.md §3.1 п. 7);
- нагрузка от `last_batch_response` вынесена в TC-D06-08.

**Не принято, с обоснованием:** ограничение шага 3 только декларациями, которые граница уже пересекла. После проверки по замкнутым интервалам вложенный слот невозможен. Восстановление внутри ещё не пересечённой декларации не меняет ни `pending_count`, ни `lw`, а запись и событие `LATE_AFTER_LOSS` сохраняются для расследования.

**Исполняемая модель ingest (SR3-01, SR3-02).** Одноразовая модель спецификации на Python, вне репозитория, не производственный код. Итог — 33 из 33 PASS:
- 5 проверок контрпримеров к ревизии 3;
- 26 детерминированных проверок ревизии 4;
- 1 проверка по 300 случайным прогонам в трёх режимах (EVENT_PIPELINE_DESIGN.md §4.10);
- 1 проверка покрытия: режим `TELEMETRY_INCOMPLETE` задействован 7050 раз, 2847 задержанных передач отброшены.

Мутационная проверка: обнаружены 5 из 5 мутаций, контрольный прогон без ошибок.

**PostgreSQL 16 (SR3-03).** Одноразовая проверка на отдельном временном кластере PostgreSQL 16.15 с синтетическими данными. SQL проверен в редакции ревизии 4, до добавления запаса `$dead_grace` в инструкцию обслуживания: S11 выполнен с запасом 0, запас меняет только порог времени. Результат — 16 из 16 проверок в 13 группах S1…S13 (EVENT_PIPELINE_DESIGN.md §5.6). Глобальные настройки и существующие базы не изменялись, тестовые объекты удалены.

Замечание о среде: в первых прогонах при удалении тестовой базы контрольная точка сервера завершилась PANIC. Причина в среде проверки — она восстанавливает права каталогов сессии между вызовами. Окончательный прогон выполнен целиком в одном вызове: без аварий, в журнале только ожидаемый `57P01` от `pg_terminate_backend`.

Затронутые файлы ревизии 4 (14): ADR-0006, ADR-0016; EVENT_PIPELINE_DESIGN.md, FAILURE_MODE_DESIGN.md; DATA_PROTECTION_DESIGN.md; DATA_MODEL.md, DECISION_LOG.md, FEATURE_REGISTRY.md, PROJECT_STATE.md, ROADMAP.md, SYSTEM_ARCHITECTURE.md, TEST_STRATEGY.md, THREAT_MODEL.md, CHANGELOG.md. D-01…D-11 не изменены; C-01…C-06 — Pending Approval; Q-01…Q-09 открыты; Proposed-параметры, в том числе новые `MAX_LOSS_SPAN`, `MARKER_RESERVE`, `MAX_INVALID_MARKERS_PER_HOUR`, `T_renew`, не утверждались.

### CHG-0003, ревизия 3 — исправления по Security Architecture Review, раунд 2 (Changes Requested)

| Замечание | Изменение |
| --- | --- |
| SR2-01 | Разделены целостность и защита от отката состояния времени; разрешающие отступления в офлайне только в текущем запуске сервиса; `T_boot_penalty`; состояние `TIME_UNVERIFIABLE`; разрывы времени; исследование TPM (DLP-ENDPOINT-020); инварианты INV-T1…T5; T-31, R-24 |
| SR2-02 | Формальный порядок проверок приёма; закрытие пропуска вне квоты и окна; окно от `low_watermark`; правило «головы очереди»; лимиты маркеров `EVENTS_LOST`; идемпотентный ACK |
| SR2-03 | Декларации потерь и восстановления только для добавления; позднее поступление — отдельная связанная запись; права БД без `UPDATE`/`DELETE` |
| SR2-04 | Неспособность исполнения отделена от неопределённости анализа: `enforcement_status`, событие `ENFORCEMENT_FAILED`, запрет ложного `enforced_action = BLOCK`; `CAPABILITY_MISMATCH` исключён из причин INDETERMINATE; T-32 |
| Уточнения 1–5 | Границы аренды и fencing (R-23); отказ локального хранилища (R-22); варианты ключа HMAC; границы crypto-shredding; устранено предупреждение `git apply` |

Затронутые файлы ревизии 3: ADR-0006, ADR-0016, ADR-0017; EVENT_PIPELINE_DESIGN.md, FAILURE_MODE_DESIGN.md, CAPABILITY_MATRIX.md; DATA_PROTECTION_DESIGN.md; DATA_MODEL.md, DECISION_LOG.md, FEATURE_REGISTRY.md, PROJECT_STATE.md, ROADMAP.md, SYSTEM_ARCHITECTURE.md, TEST_STRATEGY.md, THREAT_MODEL.md, CHANGELOG.md. D-01…D-11 и статусы C-01…C-06 не изменены; Proposed-параметры не утверждались.

### CHG-0003, ревизия 2 — исправления по архитектурному ревью от 2026-11-10 (Changes Requested)

| Замечание | Изменение |
| --- | --- |
| AR-01 | Сериализация приёма одного `(endpoint_id, agent_epoch)` блокировкой `endpoint_seq_state`; правила для перекрывающихся и переупорядоченных пакетов, конфликтов содержимого, скачков `seq`, маркеров `EVENTS_LOST`; атомарное продвижение `low_watermark` |
| AR-02 | Атомарный захват заданий, восстановление по истёкшей аренде, fencing по `lease_token`, идемпотентность побочных действий, ограничение для внешних систем |
| AR-03 | Источники доверенного времени, сохраняемый накопитель времени, защита от отката часов, состояние `TIME_UNVERIFIABLE`, восстановление после изоляции, аварийное восстановление; поведение после `hard_expiry_at` оставлено Proposed |
| AR-04 | Границы гарантий дедупликации после очистки квитанций, данные для расследований, выявление аномальных повторов; `EVENTS_LOST` — заявление агента |
| AR-05 | Разделение `required_action`, `enforced_action`, `channel_control_state`, `observation`; обнаружение потери контроля; исправлен TC-D08-03, добавлены TC-D08-06 и TC-D08-07 |
| AR-06 | Уровни L1, L2, L3 для USB; новая функция DLP-ENDPOINT-019 (L3); R-15 сохранён |
| AR-07 | Полный путь файла не передаётся по умолчанию: класс расположения и HMAC; уровень `FULL` по правилу и праву `incident.view_path` |
| AR-08 | Ключи периода KEK_P, сроки полного вступления удаления в силу, уничтожение ключевого материала, журнал удалений при восстановлении |
| AR-09 | Исправлена нумерация шагов в отдельном предложении по CLAUDE.md; CLAUDE.md не изменён |

Затронутые файлы ревизии 2: ADR-0006, ADR-0016, ADR-0017 (уточнения, статусы не изменены); EVENT_PIPELINE_DESIGN.md, FAILURE_MODE_DESIGN.md, CAPABILITY_MATRIX.md, DECISION_PROPOSAL_C01_C06.md; DATA_PROTECTION_DESIGN.md; DATA_MODEL.md, FEATURE_REGISTRY.md, THREAT_MODEL.md, TEST_STRATEGY.md, DECISION_LOG.md, PROJECT_STATE.md, CHANGELOG.md. C-01…C-06 остаются Pending Approval.

## CHG-0002 — Согласование базовой архитектуры (D-01…D-04)

| Поле | Значение |
| --- | --- |
| Change ID | CHG-0002 (подготовлено как CHG-0001; перенумеровано 2026-11-10 из-за коллизии с FID/CHG-0001, см. RES-06) |
| Дата и версия | 2026-11-10; версия продукта не присвоена (кода нет) |
| Инициатор | Владелец продукта — согласование ARCH-0001, раздел 17 |
| Что было | Только CLAUDE.md, README.md, docs/PROJECT_STATE.md; референсом указан yt-dlp |
| Что изменено | Добавлены ADR-0001…0004, DECISION_LOG.md, REFERENCE_ANALYSIS.md, CHANGELOG.md; обновлён PROJECT_STATE.md |
| Причина | Фиксация согласованных архитектурных решений перед началом реализации |
| Затронутые компоненты | Все (архитектурные решения); файлы — только docs/ |
| Связанные требования | CLAUDE.md §6, §7, §13, §14 |
| Изменения API / БД | Нет |
| Риски | Решение D-04 условно до ответа на Q-02 |
| Совместимость | Не применимо |
| Миграции | Нет |
| Результаты тестирования | Не применимо: изменения только в документации |
| Откат | `git revert` коммита с этими файлами после фиксации; до фиксации — отказ от применения патча |
| Статус | Подготовлено локально как патч; в Git не зафиксировано. **Обновление 2026-11-12:** опубликовано вместе с CHG-0003 (PR #1) и влито в `develop` |

## CHG-0001 — Первоначальная документация в ветке feature/initial-documentation (историческая запись)

| Поле | Значение |
| --- | --- |
| Change ID | CHG-0001 (в документах обозначается FID/CHG-0001) |
| Дата | 2026-11-09 |
| Источник | Коммит d184460 в ветке `feature/initial-documentation`; автор — владелец репозитория |
| Содержание | PRODUCT_REQUIREMENTS, SYSTEM_ARCHITECTURE, SECURITY_ARCHITECTURE, FEATURE_REGISTRY (106 функций), ROADMAP, CHANGELOG, PROJECT_STATE |
| Отношение к основной линии | По решению владельца продукта от 2026-11-10 основным комплектом является ARCH-0001; ветка — исторический источник требований |
| Статус | Не влито в `develop` и `main`; ветка сохраняется без изменений и не удаляется |
| Сопоставление | docs/architecture/DOCUMENTATION_RECONCILIATION.md (RECON-0001) |
