# Sentinel Core — каноническое представление `Decision` и `InspectionResult`

| Атрибут | Значение |
| --- | --- |
| Назначение | Контракт канонического представления для проверки детерминизма (SENTINEL_CORE_DESIGN.md §6.5, TC-CORE-15). Разделяет требования, принятые ранее, и требования CHG-0008 |
| Версия | 1.3 (ревизия 4 CHG-0008: шапка — актуальный статус проверок, CHG8-IMPL-07; 1.2 — ревизия 3 CHG-0008: раздел 13 — эталоны D в тестах; 1.1 — ревизия 2 CHG-0008: раздел 13 — эталоны D и ссылки на результаты; 1.0 — CHG-0008: текст разделов 1–12 — утверждённый проект 0.2.1 без изменения содержания; ссылки «§N» на проектный документ уточнены как ссылки на SENTINEL_CORE_DESIGN.md, ссылки на разделы RFC записаны словом «раздел»; добавлен раздел 13 о реализации) |
| Дата изменения | 2026-11-15 |
| Статус | **Accepted** — решения владельца CD-1…CD-8 от 2026-11-13 (объект — внешний пакет CHG-0008 v0.2, SHA-256 `edf52ac51b61851c2e8b9c1e6e9faa9841fd8fb20a3bbdf9bea6c39f7dcedf51`; запись — пакет v0.2.1, SHA-256 `fd3df845e964a2353bab3a8c30fefeb6298e018a80650fbe883a43da529b99ce`; DECISION_LOG.md §7). Реализовано в CHG-0008 локально (раздел 13), не зафиксировано. Независимое ревью ревизии 2 и F-05 (вторая ОС, Windows GNU, Rust 1.97.0) — PASS в границах TEST_RESULTS.md TR-0011: только «canonical + PDP на фиксированных `InspectionResult`»; контроль завершения ревизий 3 и 4 — отдельный статус (TR-0013, TR-0014). Полный TC-CORE-15 (извлечение, детекторы) — NOT VERIFIED (KNOWN_ISSUES.md KI-12) |
| Исходный объект проекта | Коммит `eba5baac29d3f520e7b97cbe39b43aaf096dbef4`, дерево `86323556a0f6063a454103f4d8a9029839782b02`. Типы — `crates/sentinel-core-types/src/{decision,inspection,reason,ids}.rs`; порядок массивов — `crates/sentinel-core-pdp/src/{decide,eval,policy}.rs` |
| Связанные требования | SENTINEL_CORE_DESIGN.md §2 (таблица crates), §3.2, §5, §6.5, §6.6, §11; SENTINEL_CORE_TEST_PLAN.md TC-CORE-15 (INV-C7); ИТ-05, ИТ-08, ИТ-09 (CHG7-OD1); ИТ-13, ИТ-22…ИТ-24 (KNOWN_ISSUES.md) |
| Связанные компоненты | `crates/sentinel-core-types/src/canonical.rs`; эталоны `crates/sentinel-core-types/tests/fixtures/canonical/`; тесты `crates/sentinel-core-types/tests/canonical_fixtures.rs`, `crates/sentinel-core-pdp/src/tests/canonical.rs`; эталоны D `crates/sentinel-core-pdp/tests/fixtures/canonical/` |
| Связанные документы | Матрица проверок — внешний пакет CHG-0008 v0.2.1 (`CHG-0008-test-matrix.md`); TEST_RESULTS.md TR-0008…TR-0014; KNOWN_ISSUES.md; DECISION_LOG.md §7 |

## 1. Назначение и границы

Каноническое представление — последовательность байтов, однозначно определяемая значением `Decision` или `InspectionResult`. Оно нужно для двух проверок:

- сравнения результата кодирования с рукописными эталонами (golden);
- побайтового сравнения одних и тех же фикстур между запусками, процессами, потоками и ОС (TC-CORE-15).

Каноническое представление **не является** форматом вывода CLI (`sentinel-scan/result/v1` — отдельная схема с другими именами и составом полей, SENTINEL_SCAN_CLI_DESIGN.md §4.2), IPC и `ExtractedContent` (O-06), форматом бандла (POLICY_ENGINE_DESIGN.md, O-05), событием для SIEM/SOAR или публичным протоколом интеграции. Использование правил этого документа где-либо ещё — отдельное решение владельца.

Реализуется только кодирование; разбор канонических байтов в CHG-0008 не входит. Это не устраняет недоверенный ввод: строки внутри кодируемых значений приходят из бандла, от хоста (`DeviationValidity.valid_ids`, `file_class` при `HOST_SYNTHESIZED`) и от будущих производителей инспекции. Кодировщик обязан точно и корректно кодировать любую строку (CF-12) — это его поверхность недоверенного ввода; её покрывают C-03 и L-01.

Каноническое представление — не проекция раскрытия данных. Оно кодирует значение целиком; безопасность содержимого определяется самим значением и проекцией `Projector`/`data_level`, а не кодированием (раздел 10).

## 2. Обозначения статусов

| Метка | Значение |
| --- | --- |
| **П** | Принято документом: SENTINEL_CORE_DESIGN.md (SENTINEL_CORE_DESIGN.md §3.2, SENTINEL_CORE_DESIGN.md §5, SENTINEL_CORE_DESIGN.md §6.6, SENTINEL_CORE_DESIGN.md §11), golden-ожидания G-xx плана испытаний, утверждённые решения владельца по ИТ (CHG7-OD1) |
| **Р** | Реализовано в этапе 1 (`eba5baac`, закрытое ревью) и наблюдается в коде, но в принятых документах не закреплено |
| **Н** | Новое требование CHG-0008. Метка обозначает происхождение требования: все требования с меткой Н утверждены владельцем 2026-11-13 решениями CD-1…CD-8 |

В таблицах раздела 5 у каждого поля два статуса: «содержание» (что поле означает и когда заполнено) и «представление» (как оно кодируется). Утверждённый `Option` — это статус содержания П («значение может отсутствовать»); способ записать отсутствие в JSON — всегда Н (CF-4, CD-2).

## 3. Общие правила (все — Н)

| ID | Правило | Обоснование; альтернатива |
| --- | --- | --- |
| CF-1 | Семейство формата — профиль JSON (RFC 8259) с правилами CF-2…CF-17 | Эталоны читаются и пишутся вручную; синтаксис проверяется стандартным парсером любого инструмента. Альтернатива — детерминированный CBOR (RFC 8949, раздел 4.2): эталоны только в шестнадцатеричном виде, ручное ревью хуже. CD-1 |
| CF-2 | Текст — UTF-8 без BOM; вне строк нет пробельных символов; нет завершающего перевода строки | Однозначность байтов. Перевод строки добавляет только внешний инструмент записи файлов, если ему так нужно — не кодировщик |
| CF-3 | Объект: ключи — ASCII `snake_case`, по возрастанию байтов UTF-8, без повторов. Для каждого вида объекта и каждого варианта перечисления набор ключей фиксирован: все ключи присутствуют всегда | Порядок проверяется механически и не зависит от порядка объявления полей в Rust (перестановка полей при рефакторинге не меняет байты). Альтернатива — порядок объявления по таблицам раздела 5. CD-3 |
| CF-4 | Поле `Option<T>`: ключ присутствует всегда; `None` → `null`; `Some(v)` → представление `v` | Фиксированный набор ключей (CF-3) упрощает рукописные эталоны и проверку формы; отсутствие ключа нельзя спутать с полем, которого не было в старой версии. Альтернатива — опускать ключ при `None`. CD-2. **Не следует из ИТ-05**: ИТ-05 утверждает только то, что значение отсутствует |
| CF-5 | `Vec<T>`: всегда массив; пустой — `[]`; элементы — в порядке значения; повторы сохраняются | Кодировщик не сортирует и не удаляет повторы (раздел 6) |
| CF-6 | Перечисление без полезной нагрузки — строка из таблицы 5.7 | Имена — из нотации SENTINEL_CORE_DESIGN.md §5, SENTINEL_CORE_DESIGN.md §3.2, SENTINEL_CORE_DESIGN.md §6.6 и `ReasonCode::as_str` |
| CF-7 | Перечисление, у которого есть вариант с полезной нагрузкой, — объект `{"kind": <вариант>, <поля варианта>}` для **всех** вариантов, включая варианты без нагрузки | Одна форма на тип; нет `null` вместо отсутствующей нагрузки |
| CF-8 | Кортежи и пары — объекты с именованными ключами (таблица 5.3) | Порядок элементов кортежа не становится скрытым контрактом |
| CF-9 | `bool` → `true` / `false` | — |
| CF-10 | Целые — десятичная запись `0` или `[1-9][0-9]*`, без знака, дробной части и экспоненты; весь диапазон типа (`u32`: 0…4294967295; `u64`: 0…18446744073709551615) | Плавающей точки в типах нет (SENTINEL_CORE_DESIGN.md §5). Значения больше 2^53 не представимы точно в числах двойной точности: инструмент проверки обязан разбирать целые точно (Python `json` — да; `JSON.parse` JavaScript — нет) |
| CF-11 | `[u8; 32]` → строка из 64 строчных шестнадцатеричных цифр, старший байт первым по порядку массива | Однозначно и читаемо |
| CF-12 | Строки: `"` → `\"`; `\` → `\\`; U+0008 → `\b`; U+0009 → `\t`; U+000A → `\n`; U+000C → `\f`; U+000D → `\r`; прочие U+0000…U+001F → `\u00XX` строчными цифрами. Все остальные символы, включая `/`, U+007F, C1 U+0080…U+009F, U+2028, U+2029, U+FEFF и символы вне BMP, — байтами UTF-8 как есть. Нормализация Unicode не выполняется | Совпадает с правилами строк RFC 8785, раздел 3.2.2.2; полного соответствия RFC 8785 не заявляю (порядок ключей и числа здесь свои). Rust `String` всегда содержит допустимый UTF-8, поэтому одиночных суррогатов не бывает. NFC и NFD одной буквы дают разные байты — это различие значений, а не дефект |
| CF-13 | Корневой объект содержит ключ `canonical_form`: `"sentinel-core/decision/canonical/v1"` или `"sentinel-core/inspection-result/canonical/v1"` | Различает корневой тип и версию правил кодирования. Версия содержания — `schema_version` (SENTINEL_CORE_DESIGN.md §11); версия правил — `canonical_form`. CD-4 касается `schema_version` у `InspectionResult` |
| CF-14 | Производные значения (`InspectionResult::object_coverage()`, `inspection_complete()`) не кодируются | Они функции закодированных полей; набор ключей совпадает с полями Rust-типа и проверяется механически. Альтернатива — кодировать как в нотации SENTINEL_CORE_DESIGN.md §5. CD-5 |
| CF-15 | Кодировщик полный, без проверки и нормализации входа (раздел 8) | CD-6 |
| CF-16 | Модуль `canonical` в `sentinel-core-types` за cargo-функцией `canonical`, выключенной по умолчанию | SENTINEL_CORE_DESIGN.md §2 относит сериализацию к `sentinel-core-types` (П). В сборке с выключенной функцией API модуля нет (проверяется L-08 для конкретной сборки). Функция — настройка сборки, а **не** граница безопасности и не мера против раскрытия: её может включить любой crate графа зависимостей (объединение функций Cargo). Она только снижает риск случайного использования (раздел 10.4). CD-7 |
| CF-17 | `RuleTrace.determining_atom` и `Explanation.deviation_contradictions` — диагностические строки без гарантии грамматики между версиями ядра; байты детерминированы при одинаковых входах и версиях | Раздел 9. CD-8 |

## 4. Корневые объекты и версии

| Корень | `canonical_form` | Версия содержания | Статус |
| --- | --- | --- | --- |
| `Decision` | `sentinel-core/decision/canonical/v1` | Поле `schema_version: u32` кодируется как есть; в этапе 1 значение `SCHEMA_VERSION = 1` | Поле — П (SENTINEL_CORE_DESIGN.md §5); значение 1 — Р; ключ `canonical_form` — Н |
| `InspectionResult` | `sentinel-core/inspection-result/canonical/v1` | **Пробел:** SENTINEL_CORE_DESIGN.md §11 требует `schema_version` у `InspectionResult`, а у Rust-типа такого поля нет (предлагаемое ИТ-22, раздел 11). Предложение CD-4(а): ключ `schema_version` из константы `INSPECTION_RESULT_SCHEMA_VERSION: u32 = 1` в `sentinel-core-types`, без изменения полей типа. В каноническом представлении значение — ровно целое `1`: проверяльщик отклоняет `0`, `2`, `4294967295` и `true` (CHG8-AR1-01). CD-4(а) решает только версию **канонического представления**; ИТ-22 для самого типа и других его представлений остаётся Open | Н |
| Вложенные объекты | Нет | Нет собственных версий | Н |

Изменение правил CF — новая версия `canonical_form` (`…/v2`). Изменение состава полей — правило SENTINEL_CORE_DESIGN.md §11 для `schema_version`. Обе версии входят в байты, поэтому сравнение байтов разных версий честно даёт «не совпадает».

## 5. Таблицы полей

Порядок строк — порядок объявления полей в Rust; порядок ключей в байтах — по CF-3. «Всегда» означает, что ключ присутствует при любом значении.

### 5.1. `Decision`

| № | Путь | Rust-тип | Ключ и представление | Присутствие; `null`; пустое | Порядок массива и источник | Статус: содержание / представление |
| --- | --- | --- | --- | --- | --- | --- |
| — | — | — | `canonical_form`: строка-константа | Всегда | — | — / Н |
| 1 | `verdict` | `Verdict` | `verdict`: строка 5.7 | Всегда | — | П SENTINEL_CORE_DESIGN.md §5 / Н |
| 2 | `reasons` | `Vec<ReasonCode>` | `reasons`: массив строк | Всегда; `[]` при ALLOW и BLOCK | При INDETERMINATE — по возрастанию порядка объявления `ReasonCode` (не по алфавиту), без повторов (`sorted_reasons`, `BTreeSet`); без бандла — один элемент | Непусто при INDETERMINATE — П SENTINEL_CORE_DESIGN.md §5; порядок — Р `decide.rs` |
| 3 | `policy_version` | `Option<u64>` | `policy_version`: целое или `null` | Всегда; `null` без проверенного бандла | — | П SENTINEL_CORE_DESIGN.md §5, ИТ-05 / Н |
| 4 | `policy_bundle_id` | `Option<BundleId>` | строка или `null` | Всегда; `null` без бандла | — | П SENTINEL_CORE_DESIGN.md §5, ИТ-05 / Н |
| 5 | `matched_rules` | `Vec<RuleId>` | массив строк | Всегда; может быть `[]` | Порядок ≻ применимых правил (`app` в порядке бандла, проверенном при загрузке) | Состав «включая проигравшие» — П SENTINEL_CORE_DESIGN.md §5; порядок — Р |
| 6 | `required_action` | `Option<Action>` | строка или `null` | Всегда; `null` только при `NO_RULE_MATCHED` | — | П SENTINEL_CORE_DESIGN.md §5, ИТ-09 / Н |
| 7 | `decision_basis` | `DecisionBasis` | строка | Всегда | — | П SENTINEL_CORE_DESIGN.md §5 / Н |
| 8 | `inspection_complete` | `bool` | `true`/`false` | Всегда | — | П SENTINEL_CORE_DESIGN.md §5 / Н |
| 9 | `winner_rule_id` | `Option<RuleId>` | строка или `null` | Всегда; не `null` только при единственном возможном победителе | — | П SENTINEL_CORE_DESIGN.md §5 / Н |
| 10 | `possible_winners` | `Vec<RuleId>` | массив строк | Всегда; `[]` при единственном победителе | По возрастанию байтов `rule_id` | П: G-15, ИТ-08 (класс A) |
| 11 | `part_exclusion_id` | `Option<RecordId>` | строка или `null` | Всегда | — | П SENTINEL_CORE_DESIGN.md §4.2, SENTINEL_CORE_DESIGN.md §5 / Н |
| 12 | `precedence_approval_ids` | `Vec<RecordId>` | массив строк | Всегда | По возрастанию байтов, без повторов (`BTreeSet`) | Состав — П SENTINEL_CORE_DESIGN.md §5; порядок — Р |
| 13 | `suppressed_precedence_pairs` | `Vec<(RuleId, RuleId)>` | массив объектов «пара» (5.3) | Всегда | По (позиция старшего правила в ≻, позиция младшего в ≻) — порядок `conflict_pairs` | Состав — П SENTINEL_CORE_DESIGN.md §5; порядок — Р `policy.rs` |
| 14 | `inactive_deviation_ids` | `Vec<RecordId>` | массив строк | Всегда | По возрастанию байтов (`sort`) | Состав — П SENTINEL_CORE_DESIGN.md §5 (ИТ-07 снято); порядок — Р |
| 15 | `deviation_input` | `DeviationInput` | строка | Всегда | — | П SENTINEL_CORE_DESIGN.md §6.8 / Н |
| 16 | `incomplete_action_source` | `Option<RecordId>` | строка или `null` | Всегда | — | П SENTINEL_CORE_DESIGN.md §5 / Н |
| 17 | `action_certainty` | `Option<ActionCertainty>` | строка или `null` | Всегда; не `null` только при ALLOW, кроме `NO_RULE_MATCHED` | — | П SENTINEL_CORE_DESIGN.md §5, ИТ-09 / Н |
| 18 | `action_sources` | `Vec<ActionSource>` | массив объектов (5.3) | Всегда; `[]` при BLOCK, INDETERMINATE и `NO_RULE_MATCHED` | Порядок ≻, запись контракта последней — П SENTINEL_CORE_DESIGN.md §5; по одному источнику на различное действие, первый по порядку — Р | П / Н |
| 19 | `verification_mode` | `Option<VerificationMode>` | строка или `null` | Всегда; `null` без бандла | — | П SENTINEL_CORE_DESIGN.md §5, ИТ-05 / Н |
| 20 | `capability_check` | `Option<CapabilityCheck>` | строка или `null` | Всегда; `null` без бандла | — | П SENTINEL_CORE_DESIGN.md §5, ИТ-05 / Н |
| 21 | `requested_action` | `Option<Action>` | строка или `null` | Всегда; не `null` только при исключении ADR-0019 | — | П SENTINEL_CORE_DESIGN.md §5 / Н |
| 22 | `capability_exception_id` | `Option<RecordId>` | строка или `null` | Всегда | — | П SENTINEL_CORE_DESIGN.md §5 / Н |
| 23 | `fallback` | `Option<Fallback>` | строка или `null` | Всегда; не `null` только при INDETERMINATE | — | П SENTINEL_CORE_DESIGN.md §5 / Н |
| 24 | `data_level` | `DataLevel` | строка | Всегда | — | П SENTINEL_CORE_DESIGN.md §5 / Н |
| 25 | `degraded` | `bool` | `true`/`false` | Всегда | — | П SENTINEL_CORE_DESIGN.md §5, ИТ-04 / Н |
| 26 | `explanation` | `Explanation` | объект 5.2 | Всегда | — | П SENTINEL_CORE_DESIGN.md §6.6 / Н |
| 27 | `schema_version` | `u32` | целое | Всегда | — | П SENTINEL_CORE_DESIGN.md §5, SENTINEL_CORE_DESIGN.md §11 / Н |

### 5.2. `Explanation` (вложен в `Decision`)

Состав объяснения — П SENTINEL_CORE_DESIGN.md §6.6; разбиение на поля — Р (`decision.rs`). Без бандла (`decide_without_policy`) незаполненные поля берутся из `Explanation::default()`: пустые массивы, `None`, `false`, `worlds_considered = 0`.

| № | Путь | Rust-тип | Ключ и представление | Присутствие; `null`; пустое | Порядок массива и источник | Статус: содержание / представление |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `core_version` | `String` | строка | Всегда; в этапе 1 — `CARGO_PKG_VERSION` (`0.1.0`) | — | П SENTINEL_CORE_DESIGN.md §6.6 / Н |
| 2 | `detector_set_version` | `String` | строка | Всегда; копия `InspectionResult.engine.detector_set_version` | — | П SENTINEL_CORE_DESIGN.md §6.6 / Н |
| 3 | `bundle_id` | `Option<BundleId>` | строка или `null` | Всегда; `null` без бандла | — | П / Н |
| 4 | `policy_version` | `Option<u64>` | целое или `null` | Всегда; `null` без бандла | — | П / Н |
| 5 | `file_class` | `Option<FileClass>` | строка или `null` | Всегда; в этапе 1 всегда `Some` | — | П SENTINEL_CORE_DESIGN.md §6.6 / Н |
| 6 | `classification` | `Option<Classification>` | объект 5.6 или `null` | Всегда | — | П / Н |
| 7 | `text_coverage` | `Option<TextCoverage>` | строка или `null` | Всегда | — | П / Н |
| 8 | `structure_enumeration` | `Option<StructureEnumeration>` | объект 5.6 или `null` | Всегда | — | П / Н |
| 9 | `inspection_complete` | `bool` | `true`/`false` | Всегда | — | П / Н |
| 10 | `detectors` | `Vec<(DetectorId, u32, bool)>` | массив объектов «след детектора» (5.3) | Всегда | Порядок `InspectionResult.detections` — по возрастанию `detector_id` | П SENTINEL_CORE_DESIGN.md §5 (сортировка `detections`) / Н |
| 11 | `rules` | `Vec<RuleTrace>` | массив объектов (5.3) | Всегда | Порядок ≻ применимых правил | Состав «каждое применимое правило» — П SENTINEL_CORE_DESIGN.md §6.6; порядок — Р |
| 12 | `winner_basis` | `Option<OrderBasis>` | строка или `null` | Всегда; не `null` только при единственном победителе, за которым есть ещё TRUE-правило | — | П SENTINEL_CORE_DESIGN.md §6.6 / Н |
| 13 | `unknown_rules` | `Vec<RuleId>` | массив строк | Всегда | Порядок ≻ (правила выше W со значением UNKNOWN) | П SENTINEL_CORE_DESIGN.md §6.6 (множество U) / Р порядок |
| 14 | `possible_winners` | `Vec<RuleId>` | массив строк | Всегда; заполнен и при одном победителе | **Порядок ≻**, а не `rule_id`: отличается от `Decision.possible_winners`. Приводить к одному порядку нельзя | Р |
| 15 | `world_without_winner` | `bool` | `true`/`false` | Всегда | — | Р |
| 16 | `worlds_considered` | `u64` | целое | Всегда; 1 без перебора; 0 без бандла | — | П SENTINEL_CORE_DESIGN.md §6.6 (число миров) / Н |
| 17 | `class_worlds` | `Vec<Option<FileClass>>` | массив объектов «мир класса» (5.3); `None` здесь — класс OTHER (SENTINEL_CORE_DESIGN.md §4.3), а не отсутствие | Всегда; `[]` при подтверждённой классификации | Классы по возрастанию байтов, затем OTHER (`BTreeSet` в `eval.rs`) | Порядок — Р; случай G-20 `[OOXML, TXT, OTHER]` — П / Н |
| 18 | `fallback_sources` | `Vec<RuleId>` | массив строк | Всегда; не пуст только при INDETERMINATE | Порядок ≻ | Р |
| 19 | `part_exclusion_id` | `Option<RecordId>` | строка или `null` | Всегда | — | П / Н |
| 20 | `precedence_approval_ids` | `Vec<RecordId>` | массив строк | Всегда | По возрастанию байтов | Р |
| 21 | `suppressed_precedence_pairs` | `Vec<(RuleId, RuleId)>` | массив объектов «пара» | Всегда | Как в `Decision` | Р |
| 22 | `incomplete_action_source` | `Option<RecordId>` | строка или `null` | Всегда | — | П / Н |
| 23 | `capability_exception_id` | `Option<RecordId>` | строка или `null` | Всегда | — | П / Н |
| 24 | `shadowed_incompleteness_guards` | `Vec<RuleId>` | массив строк | Всегда | Порядок ≻ (TRUE-правила с атомом неполноты ниже победителя) | П SENTINEL_CORE_DESIGN.md §6.6 / Р порядок |
| 25 | `deviation_contradictions` | `Vec<String>` | массив диагностических строк (CF-17) | Всегда; не пуст только при `CONTRADICTORY` | Сначала сообщение о `TIME_UNVERIFIABLE`, затем неизвестные идентификаторы по возрастанию байтов (`normalize_validity`) | Р |
| 26 | `policy_state` | `Option<ReasonCode>` | строка или `null` | Всегда; не `null` только без бандла | — | Р / Н |

### 5.3. Вложенные объекты `Decision`

| Rust | Ключи и представление | Источник имён | Статус |
| --- | --- | --- | --- |
| `ActionSource { source, action }` | `{"action": строка Action, "source": объект ActionSourceRef}` | Поля Rust; нотация SENTINEL_CORE_DESIGN.md §5 `{ source, action }` | П состав / Н |
| `ActionSourceRef::Rule(RuleId)` | `{"kind":"RULE","rule_id": строка}` | SENTINEL_CORE_DESIGN.md §5 `source: RuleId \| IncompleteInspectionActionId` | Н |
| `ActionSourceRef::IncompleteInspectionAction(RecordId)` | `{"kind":"INCOMPLETE_INSPECTION_ACTION","record_id": строка}` | То же | Н |
| `RuleTrace { rule_id, value, determining_atom, guards_incompleteness }` | `{"determining_atom": строка или null, "guards_incompleteness": bool, "rule_id": строка, "value": строка Truth}` | Поля Rust | П SENTINEL_CORE_DESIGN.md §6.6 состав / Н |
| `(RuleId, RuleId)` в `suppressed_precedence_pairs` | `{"higher_rule_id": строка, "lower_rule_id": строка}` | Первый элемент — старшее правило пары, второй — младшее (`conflict_pairs`, SENTINEL_CORE_DESIGN.md §6.3 шаг 2) | Н |
| `(DetectorId, u32, bool)` в `Explanation.detectors` | `{"detector_id": строка, "match_count": целое u32, "threshold_met": bool}` | Значения берутся из `DetectionResult.detector_id`, `.match_count`, `.threshold_met` (`decide.rs`) | Н |
| `Option<FileClass>` в `class_worlds` | `Some(c)` → `{"file_class": c, "kind":"CLASS"}`; `None` → `{"kind":"OTHER"}` | OTHER — SENTINEL_CORE_DESIGN.md §4.3, комментарий `eval.rs::worlds` | Н. `null` не используется: здесь `None` — значение «другой класс», а не отсутствие. Строка `"OTHER"` тоже не годится — она неотличима от класса с таким именем |

### 5.4. `InspectionResult`

Состав — П SENTINEL_CORE_DESIGN.md §5 (кроме `inspection_origin`: П SENTINEL_CORE_DESIGN.md §10.2 и таблица полей SENTINEL_CORE_DESIGN.md) и ИТ-01 (`file_class`). Поля `object_coverage` и `inspection_complete` SENTINEL_CORE_DESIGN.md §5 — производные методы (ИТ-02) и по CF-14 не кодируются. `findings` в Rust-типе нет.

| № | Путь | Rust-тип | Ключ и представление | Присутствие; `null`; пустое | Порядок массива и источник | Статус: содержание / представление |
| --- | --- | --- | --- | --- | --- | --- |
| — | — | — | `canonical_form`: строка-константа | Всегда | — | — / Н |
| — | — | константа (CD-4а) | `schema_version`: целое | Всегда | — | Требование П SENTINEL_CORE_DESIGN.md §11; источник значения — Н |
| 1 | `status` | `InspectionStatus` | строка | Всегда | — | П SENTINEL_CORE_DESIGN.md §5 / Н |
| 2 | `reasons` | `Vec<ReasonCode>` | массив строк | Всегда; не пуст при `NOT_INSPECTED` и неполноте (проверяет `check_contract`) | **Не определён** ни документом, ни кодом: порядок производителя сохраняется как есть. Требование к будущим производителям — предлагаемое ИТ-23 | П состав / пробел порядка |
| 3 | `text_coverage` | `TextCoverage` | строка | Всегда | — | П SENTINEL_CORE_DESIGN.md §4.1 / Н |
| 4 | `structure_enumeration` | `StructureEnumeration` | объект 5.6 | Всегда | — | П SENTINEL_CORE_DESIGN.md §4.2 / Н |
| 5 | `classification` | `Classification` | объект 5.6 | Всегда | — | П SENTINEL_CORE_DESIGN.md §4.3 / Н |
| 6 | `uninspected_parts` | `Vec<UninspectedPart>` | массив объектов 5.5 | Всегда | **Не определён**: порядок и уникальность `kind` не заданы; сохраняется как есть (ИТ-23) | П состав / пробел порядка |
| 7 | `detections` | `Vec<DetectionResult>` | массив объектов 5.5 | Всегда | Строго по возрастанию байтов `detector_id` | П SENTINEL_CORE_DESIGN.md §5; `check_contract` — `DetectionsNotSorted` |
| 8 | `file_class` | `FileClass` | строка | Всегда | — | П ИТ-01 / Н |
| 9 | `digest` | `ContentDigest` | объект 5.5 | Всегда | — | П SENTINEL_CORE_DESIGN.md §3.2 / Н |
| 10 | `engine` | `EngineInfo` | объект 5.5 | Всегда | — | П SENTINEL_CORE_DESIGN.md §5 / Н |
| 11 | `inspection_origin` | `InspectionOrigin` | строка | Всегда | — | П SENTINEL_CORE_DESIGN.md §10.2 / Н |

### 5.5. Вложенные объекты `InspectionResult`

| Rust | Ключи и представление | Статус |
| --- | --- | --- |
| `DetectionResult { detector_id, detector_version, match_count: u32, score: u32, threshold_met }` | `{"detector_id": строка, "detector_version": строка, "match_count": целое, "score": целое, "threshold_met": bool}` | П SENTINEL_CORE_DESIGN.md §5 / Н. `score` кодируется как значение; порогов по `score` в модели v0.1 нет (CHG7-OD2, KI-08), кодирование это не меняет |
| `UninspectedPart { kind, count: u32 }` | `{"count": целое, "kind": строка PartKind}` | П SENTINEL_CORE_DESIGN.md §5 / Н |
| `ContentDigest { full: Option<[u8;32]>, full_status, read_set_digest: [u8;32], trust }` | `{"full": 64 hex или null, "full_status": строка DigestStatus, "read_set_digest": 64 hex, "trust": строка DigestTrust}` | П SENTINEL_CORE_DESIGN.md §3.2 / Н. В SENTINEL_CORE_DESIGN.md §3.2 записано `COMPUTED \| NOT_COMPUTED(причина)`; Rust-тип плоский, представление следует Rust-типу (информационно, раздел 11) |
| `EngineInfo { core_version, detector_set_version, limits_profile_id }` | `{"core_version": строка, "detector_set_version": строка, "limits_profile_id": строка}` | П SENTINEL_CORE_DESIGN.md §5 / Н |

### 5.6. Перечисления с полезной нагрузкой (CF-7)

| Rust | Вариант → объект | Статус |
| --- | --- | --- |
| `StructureEnumeration` | `Complete` → `{"kind":"COMPLETE"}`; `Incomplete(r)` → `{"kind":"INCOMPLETE","reason": строка ReasonCode}` | П SENTINEL_CORE_DESIGN.md §4.2 / Н |
| `Classification` | `Confirmed` → `{"kind":"CONFIRMED"}`; `Ambiguous(a)` → `{"kind":"AMBIGUOUS","reason": строка AmbiguityReason}` | П SENTINEL_CORE_DESIGN.md §4.3 / Н |
| `ActionSourceRef` | 5.3 | П SENTINEL_CORE_DESIGN.md §5 / Н |

### 5.7. Перечисления без полезной нагрузки (CF-6)

Все строки — заглавные буквы и `_`. Источник имени: нотация принятых документов (П) — сама запись в JSON строкой (Н).

| Rust-тип | Вариант → строка | Источник имён |
| --- | --- | --- |
| `Verdict` | `Allow` → `ALLOW`; `Block` → `BLOCK`; `Indeterminate` → `INDETERMINATE` | SENTINEL_CORE_DESIGN.md §5 |
| `Action` | `Monitor` → `MONITOR`; `Alert` → `ALERT`; `Block` → `BLOCK` | SENTINEL_CORE_DESIGN.md §5 |
| `Fallback` | `AllowWithAudit` → `ALLOW_WITH_AUDIT`; `Block` → `BLOCK` | SENTINEL_CORE_DESIGN.md §5 |
| `DecisionBasis` | `RuleMatched` → `RULE_MATCHED`; `NoRuleMatched` → `NO_RULE_MATCHED`; `OutcomeIndependentOfUnknown` → `OUTCOME_INDEPENDENT_OF_UNKNOWN`; `Indeterminate` → `INDETERMINATE` | SENTINEL_CORE_DESIGN.md §5 |
| `ActionCertainty` | `Definite` → `DEFINITE`; `Possible` → `POSSIBLE` | SENTINEL_CORE_DESIGN.md §5 |
| `VerificationMode` | `EnforcementReady` → `ENFORCEMENT_READY`; `InspectionOnly` → `INSPECTION_ONLY` | SENTINEL_CORE_DESIGN.md §5 |
| `CapabilityCheck` | `Performed` → `PERFORMED`; `Skipped` → `SKIPPED` | SENTINEL_CORE_DESIGN.md §5 |
| `DataLevel` | `Metadata` → `METADATA`; `MaskedMatch` → `MASKED_MATCH` | SENTINEL_CORE_DESIGN.md §5 |
| `DeviationInput` | `Provided` → `PROVIDED`; `Unavailable` → `UNAVAILABLE`; `Contradictory` → `CONTRADICTORY` | SENTINEL_CORE_DESIGN.md §5, SENTINEL_CORE_DESIGN.md §6.8 |
| `Truth` | `True` → `TRUE`; `False` → `FALSE`; `Unknown` → `UNKNOWN` | SENTINEL_CORE_DESIGN.md §6.6 |
| `OrderBasis` | `Priority` → `PRIORITY`; `Strictness` → `STRICTNESS`; `RuleId` → `RULE_ID` | SENTINEL_CORE_DESIGN.md §6.6 называет основания словами; строки — Н |
| `ReasonCode` | `Timeout` → `TIMEOUT`; `UnsupportedFormat` → `UNSUPPORTED_FORMAT`; `ContentEncrypted` → `CONTENT_ENCRYPTED`; `ResourceExhausted` → `RESOURCE_EXHAUSTED`; `ParseError` → `PARSE_ERROR`; `EngineError` → `ENGINE_ERROR`; `PolicyAbsent` → `POLICY_ABSENT`; `PolicyInvalid` → `POLICY_INVALID`; `PolicyExpired` → `POLICY_EXPIRED`; `ContentChanged` → `CONTENT_CHANGED` | `ReasonCode::as_str` (Р), FAILURE_MODE_DESIGN.md §2.3 |
| `InspectionStatus` | `Match` → `MATCH`; `NoMatch` → `NO_MATCH`; `NotInspected` → `NOT_INSPECTED` | SENTINEL_CORE_DESIGN.md §5 |
| `TextCoverage` | `Complete` → `COMPLETE`; `Partial` → `PARTIAL` | SENTINEL_CORE_DESIGN.md §4.1 |
| `AmbiguityReason` | `DeclaredContainerMismatch` → `DECLARED_CONTAINER_MISMATCH`; `EmbeddedSignature` → `EMBEDDED_SIGNATURE` | SENTINEL_CORE_DESIGN.md §4.3 |
| `PartKind` | `Image` → `IMAGE`; `EmbeddedObject` → `EMBEDDED_OBJECT`; `Attachment` → `ATTACHMENT`; `Unknown` → `UNKNOWN` | SENTINEL_CORE_DESIGN.md §5 |
| `InspectionOrigin` | `Core` → `CORE`; `HostSynthesized` → `HOST_SYNTHESIZED` | SENTINEL_CORE_DESIGN.md §10.2, эскиз CLI |
| `DigestStatus` | `Computed` → `COMPUTED`; `PartialRead` → `PARTIAL_READ`; `SizeLimit` → `SIZE_LIMIT`; `ReadError` → `READ_ERROR`; `NotRequested` → `NOT_REQUESTED`; `NoSnapshot` → `NO_SNAPSHOT` | SENTINEL_CORE_DESIGN.md §3.2 |
| `DigestTrust` | `TrustedSnapshot` → `TRUSTED_SNAPSHOT`; `Untrusted` → `UNTRUSTED` | SENTINEL_CORE_DESIGN.md §3.2 |

Строки кодируются функциями сопоставления в модуле `canonical`, а не через `Debug`: вывод `Debug` не контракт.

## 6. Порядок массивов: сводка и запрет сортировки

Кодировщик выводит элементы ровно в порядке значения. Он не сортирует, не удаляет повторы и не «исправляет» порядок. Причины:

- порядок частично задан контрактом и различается по полям: `Decision.possible_winners` — по `rule_id` (G-15), `Explanation.possible_winners` — в порядке ≻; `Decision.reasons` — по порядку объявления `ReasonCode`, а не по алфавиту строк (`TIMEOUT` раньше `CONTENT_CHANGED`). Единая сортировка в кодировщике нарушила бы оба;
- если в производителе появится недетерминированный порядок (например, обход `HashMap`), сортировка в кодировщике скрыла бы именно тот дефект, который должен ловить TC-CORE-15.

Сводка источников порядка:

| Порядок | Поля | Источник |
| --- | --- | --- |
| По возрастанию `rule_id` | `Decision.possible_winners` | П G-15 |
| По возрастанию `detector_id` (строго) | `InspectionResult.detections`, `Explanation.detectors` | П SENTINEL_CORE_DESIGN.md §5 |
| Порядок ≻, запись контракта последней | `Decision.action_sources` | П SENTINEL_CORE_DESIGN.md §5 |
| Порядок ≻ | `matched_rules`, `Explanation.rules`, `unknown_rules`, `Explanation.possible_winners`, `fallback_sources`, `shadowed_incompleteness_guards` | Р |
| По (старший, младший) в ≻ | `suppressed_precedence_pairs` (оба) | Р |
| По возрастанию байтов идентификатора | `precedence_approval_ids` (оба), `inactive_deviation_ids` | Р |
| По порядку объявления `ReasonCode`, без повторов | `Decision.reasons` | Р |
| Классы по возрастанию, затем OTHER | `class_worlds` | Р; G-20 — П |
| Сообщение о времени, затем идентификаторы по возрастанию | `deviation_contradictions` | Р |
| Не определён | `InspectionResult.reasons`, `uninspected_parts` | Пробел, ИТ-23 |

Порядки, отмеченные Р, предлагается закрепить в SENTINEL_CORE_DESIGN.md §5–SENTINEL_CORE_DESIGN.md §6.6 записью CHG-0008 (Н); до этого они — наблюдаемое поведение этапа 1, которое golden-эталоны фиксируют.

## 7. Числа, строки, хеши — граничные случаи

| Случай | Представление | Эталон |
| --- | --- | --- |
| `u64::MAX` (`policy_version`, `worlds_considered`) | `18446744073709551615` | F-D1 |
| 2^53 + 1 (`Explanation.policy_version`) | `9007199254740993` — проверка точного разбора целых инструментом | F-D1 |
| `u32::MAX` (`schema_version`, `match_count`, `score`, `count`) | `4294967295` | F-D1, F-I1 |
| 0 | `0` | F-D2, F-I1, F-I2 |
| Пустая строка (типы идентификаторов её допускают) | `""` | F-D2, F-D3, F-I2 |
| `"`, `\`, `/` | `\"`, `\\`, `/` | F-D3 |
| U+0000, U+0001, U+001F | `\u0000`, `\u0001`, `\u001f` | F-D3 |
| U+0008, U+0009, U+000A, U+000C, U+000D | `\b`, `\t`, `\n`, `\f`, `\r` | F-D3, F-I1 |
| U+007F, U+0080, U+2028, U+FEFF, U+1F600 | Байты UTF-8 как есть | F-D3, F-I1 |
| NFC `é` и NFD `e` + U+0301 | `C3 A9` и `65 CC 81` — разные байты | F-D3 |
| Кириллица в `deviation_contradictions` | Байты UTF-8 как есть | F-D1, F-D3 |
| Хеш | 64 строчные шестнадцатеричные цифры; `full = None` → `null`; `read_set_digest` при `not_computed` — 64 нуля | F-I1, F-I2 |

## 8. Допустимость входа, сигнатуры, ошибки

### 8.1. Варианты

| Вариант | Поведение | Оценка |
| --- | --- | --- |
| **A. Полный кодировщик (рекомендуется, CD-6)** | Кодирует любое значение типа: без проверки инвариантов и без нормализации | Отражает ровно то, что выдал производитель; дефект PDP виден в байтах, а не скрыт отказом кодирования. Проверки инвариантов уже есть отдельно: `InspectionResult::check_contract`, тесты INV-* этапа 1 |
| B. Кодировщик с проверкой | `Result<_, CanonicalError>`, отказ при нарушении `check_contract` или INV-* | Смешивает проверку контракта с представлением; нельзя закодировать неверное значение для диагностики; появляются варианты ошибок, недостижимые для корректного PDP |
| C. Нормализующий кодировщик | Сортирует, удаляет повторы, приводит Unicode | Отклоняется: нарушает контрактный порядок (раздел 6) и скрывает недетерминизм |

### 8.2. Предлагаемые сигнатуры (Н, имена — на утверждение)

```rust
// crates/sentinel-core-types/src/canonical.rs, только при cargo-функции `canonical` (CF-16)
pub const INSPECTION_RESULT_SCHEMA_VERSION: u32 = 1; // при решении CD-4(а)

pub fn encode_decision(decision: &Decision) -> Vec<u8>;
pub fn encode_inspection_result(result: &InspectionResult) -> Vec<u8>;
```

| Свойство | Обоснование |
| --- | --- |
| Заимствование `&T` | Кодирование не владеет значением и не меняет его |
| `Vec<u8>`, а не `String` | Контракт — байты. Результат всегда допустимый UTF-8, но сравниваются байты |
| Без `Result` | Каждое значение типа представимо: `String` — всегда UTF-8; целые укладываются в CF-10 по типу; плавающей точки нет; ввода-вывода нет. Нехватка памяти в Rust завершает процесс (стандартное поведение `Vec`), это документируется. Если в типах появится непредставимое значение, сигнатура меняется вместе с новой версией `canonical_form` |
| Без паники | Нет `unwrap`, индексирования по внешним данным и арифметики с переполнением; `unsafe` запрещён на уровне workspace |
| Свободные функции, без трейтов и `serde` | Только стандартная библиотека (внешние crates в среде недоступны, решение 2026-11-12); нет неявной сериализации через `Debug`/`Display` |
| Нет функции для `ProvisionalDecision` | SENTINEL_CORE_DESIGN.md §5: «не сериализуется»; попытка передать его в `encode_decision` не компилируется (compile_fail-доктест) |
| Нет разбора | Раздел 1 |

## 9. `determining_atom` и `deviation_contradictions` (CF-17)

| Поле | Что содержит в этапе 1 | Источник строк |
| --- | --- | --- |
| `RuleTrace.determining_atom` | Текст атома, определившего итог правила: `detector(<d>).threshold_met`, `detector(<d>).count >= <n>`, `inspection.incomplete`, `inspection.incomplete(<REASON>)`, `file.class = <c>`, `context.channel = <c>`, `context.operation = <o>` (`policy.rs`, реализация `Display`) | Константы атомов политики (идентификаторы детекторов, числа, классы, каналы, операции) и коды причин; не значения контекста |
| `Explanation.deviation_contradictions` | Сообщения на русском языке: «TIME_UNVERIFIABLE при непустом valid_ids»; «идентификатор <id> не является записью-отступлением загруженного бандла» | Шаблон ядра и **идентификаторы, переданные хостом** в `DeviationValidity.valid_ids`, которых нет в бандле |

Предложение (CD-8): оба поля — диагностические строки. Их грамматика и текст между версиями ядра не гарантируются, разбирать их нельзя, сравнивать — только при равных `core_version`. При одинаковых входах и одинаковых версиях байты обязаны совпадать — диагностический статус не ослабляет детерминизм. Текст `deviation_contradictions` — на русском языке и содержит строки хоста; это фиксируется как предлагаемое ИТ-24, а не меняется в CHG-0008.

## 10. Раскрытие данных: разделение ответственности

Кодировщик строки не фильтрует. Ниже разделены четыре разные вещи; ни одна не заменяет другую, и ни одна не является проекцией данных или проверкой разрешённого уровня раскрытия.

### 10.1. Точное сохранение входного значения — обязанность кодировщика

Кодировщик воспроизводит значение целиком и без изменений: каждое строковое поле — ровно то, что передано (CF-12), без фильтрации, усечения, маскирования и удаления «подозрительных» значений. Если производитель поместил в строковое поле путь, имя файла или фрагмент содержимого, это окажется в канонических байтах. Так задумано: фильтрация сделала бы байты неточными и скрыла бы дефект производителя. Проверка — L-01: каждый маркер выводится ровно один раз и только по своему пути.

### 10.2. Обязанности производителей и проекции данных

Что попадает в значение, определяют производители, а что покидает процесс — проекция:

| Участник | Обязанность |
| --- | --- |
| PDP (`decide`, `decide_without_policy`) | Заполняет поля из бандла, `InspectionResult` и `DeviationValidity` по SENTINEL_CORE_DESIGN.md §6.3, SENTINEL_CORE_DESIGN.md §6.6. По чтению кода `eba5baac`: субъект, устройство, группы, подразделения, значения канала и операции из `DecisionContext` и области правил в `Decision` не копируются; константы канала и операции попадают только в `determining_atom` — из политики; строки хоста — только в `deviation_contradictions` (S5). Это свойства текущего кода PDP, подтверждаемые L-02…L-05 на выбранных входах, а не свойства формата |
| Производители `InspectionResult` (будущие `identify`/`extract`/`detect`; сейчас — тесты и хост при `HOST_SYNTHESIZED`) | Не помещать содержимое, пути, имена файлов и сведения о субъекте в строковые поля. При `HOST_SYNTHESIZED` `file_class` — строка хоста, ядро её не проверяет |
| Хост | Передаёт `DecisionContext`, `DeviationValidity`; может синтезировать `InspectionResult` |
| Проекция `Projector` / `data_level` | Решает, что покидает процесс (SENTINEL_CORE_DESIGN.md §5, SENTINEL_CORE_DESIGN.md §6.6). Каноническое представление проекцией не является и её не заменяет |

`InspectionResult` содержит `digest` — хеши содержимого. Хеш не раскрывает текст, но позволяет подтвердить догадку о содержимом с малой энтропией, поэтому канонический `InspectionResult` — материал с ограниченным доступом, как и сам результат инспекции.

Перечень разрешённых полей не доказывает отсутствия утечек: строковое поле может перенести любое значение. Таблица ниже — **ожидаемые** источники строк, то есть обязательства производителей, а не гарантия кодировщика. Проверки L-01…L-05 (CHG-0008-test-matrix.md) подтверждают их контрольными маркерами на выбранных входах.

| Источник | Поля |
| --- | --- |
| S1. Бандл политики (идентификаторы и константы атомов) | `matched_rules`, `winner_rule_id`, `possible_winners` (оба), `part_exclusion_id` (оба), `precedence_approval_ids` (оба), `suppressed_precedence_pairs` (оба), `inactive_deviation_ids`, `incomplete_action_source` (оба), `action_sources[].source`, `capability_exception_id` (оба), `policy_bundle_id`, `Explanation.bundle_id`, `rules[].rule_id`, `rules[].determining_atom`, `unknown_rules`, `fallback_sources`, `shadowed_incompleteness_guards`, `class_worlds[].file_class` |
| S2. Ядро и сборка | `Explanation.core_version`, `canonical_form`, шаблоны `determining_atom` и `deviation_contradictions` |
| S3. Набор детекторов и профиль лимитов | `detector_set_version` (оба), `engine.*`, `detections[].detector_id`, `detections[].detector_version`, `Explanation.detectors[].detector_id` |
| S4. Идентификация (класс по сигнатуре) или хост при `HOST_SYNTHESIZED` | `InspectionResult.file_class`, `Explanation.file_class` |
| S5. Хост: `DeviationValidity.valid_ids` вне бандла | `Explanation.deviation_contradictions` |
| S6. Содержимое (только как хеш) | `digest.full`, `digest.read_set_digest` |

В Rust-типах нет полей для байтов содержимого, `findings`, значений совпадений, путей и имён файлов. Это не означает, что такие данные не могут оказаться в выводе: строковые поля источников S1, S3, S4 и S5 принимают любые строки, и их содержимое определяют производители.

### 10.3. Запрет использования канонических байтов вне проверок — организационное требование

Канонические байты создаются только в тестах и средствах проверки детерминизма. Их нельзя писать в события, журналы, SIEM/SOAR, вывод CLI, отчёты пользователю и передавать по сети. Это требование процесса и ревью, а не техническая гарантия. Соблюдение проверяется L-08d (поиск использования вне тестов) и ревью каждого CHG; нарушение — дефект.

### 10.4. Отсутствие API в конкретной сборке

В сборке, где функция `canonical` выключена, модуля и функций нет. L-08a (отрицательная проверка без функции), L-08b (положительная с функцией) и L-08c (функция не включается через `[dependencies]`) подтверждают это для конкретной конфигурации сборки. Cargo объединяет функции: любой crate графа зависимостей, включивший `canonical`, включает его для всей сборки. Поэтому функция не граница безопасности, не учитывается как мера против раскрытия и только снижает риск случайного использования.

## 11. Выявленные пробелы (предложения для записи; не применены)

| Предлагаемый ID | Наблюдение | Предложение |
| --- | --- | --- |
| ИТ-22 | SENTINEL_CORE_DESIGN.md §11 требует `schema_version` у `InspectionResult`; в Rust-типе поля нет | CD-4(а) закрывает только версию канонического представления (константа и ключ, ровно `1`). ИТ-22 остаётся **Open**: версия самого типа и его других представлений (события, IPC, будущие форматы) — отдельным решением |
| ИТ-23 | Порядок и уникальность `InspectionResult.reasons` и `uninspected_parts` не заданы ни документом, ни `check_contract` | Кодировщик сохраняет порядок. Требование к производителям (например, `reasons` — по порядку объявления без повторов, `uninspected_parts` — по `PartKind` без повторов) — решение для этапа `extract`/`ident`, не CHG-0008 |
| ИТ-24 | `deviation_contradictions` — свободный текст на русском языке со строками хоста | CD-8: диагностический статус; структурирование — отдельный CHG |
| — (информационно) | `Explanation.possible_winners` и `Decision.possible_winners` упорядочены по-разному | Оба порядка сохраняются; закрепить в SENTINEL_CORE_DESIGN.md §6.6 |
| — (информационно) | `worlds_considered = 0` без бандла и ≥ 1 с бандлом | Закрепить в SENTINEL_CORE_DESIGN.md §6.6 |
| — (информационно) | Нотация `DigestStatus` в SENTINEL_CORE_DESIGN.md §3.2 вложенная, Rust-тип плоский | Каноническая форма следует Rust-типу |

Номера ИТ-22…ИТ-24 предварительные; присваиваются при записи в KNOWN_ISSUES.md.

## 12. Не входит в CHG-0008

Разбор канонических байтов; формат и схема CLI; IPC и `ExtractedContent` (O-06); формат бандла и подпись (O-05); изменение полей `Decision`, `InspectionResult` и алгоритма PDP; пороги по `score` (CHG7-OD2, KI-08: score-порог нельзя трактовать как count-порог); внешние crates; CI.

## 13. Реализация (CHG-0008)

| Требование | Реализация |
| --- | --- |
| CF-1…CF-14, таблицы раздела 5 | `crates/sentinel-core-types/src/canonical.rs`: функции записи на стандартной библиотеке; ключи каждого объекта перечислены в коде в порядке возрастания байтов |
| CF-15, раздел 8 | `encode_decision(&Decision) -> Vec<u8>`, `encode_inspection_result(&InspectionResult) -> Vec<u8>` — полные, без проверки инвариантов и нормализации, без `Result`; разбора нет |
| CD-4 (а) | `INSPECTION_RESULT_SCHEMA_VERSION: u32 = 1` в модуле `canonical`; поля `InspectionResult` не менялись |
| CF-13 | Константы `DECISION_CANONICAL_FORM`, `INSPECTION_RESULT_CANONICAL_FORM` — дополнительно к сигнатурам раздела 8.2 |
| CF-16, CD-7 | Cargo-функция `canonical` в `sentinel-core-types`, выключена по умолчанию; `sentinel-core-pdp` включает её только в `[dev-dependencies]`. Не граница безопасности (раздел 10.4) |
| `ProvisionalDecision` | Функции для него нет; compile_fail-доктест и проба C-22b |
| Эталоны | 20 рукописных эталонов пакета v0.2.1 скопированы побайтно в `crates/sentinel-core-types/tests/fixtures/canonical/` |
| Раздел 11 | ИТ-22…ИТ-24 записаны в KNOWN_ISSUES.md (версия 8) под теми же номерами; текст раздела 11 сохранён как утверждённый |
| Рукописные эталоны уровня PDP (D-01…D-12) | Ревизия 1: не подготовлены. Ревизия 2: набраны вручную, независимо сверены 15/15 до сравнения и закреплены по SHA-256. Ревизия 3: `crates/sentinel-core-pdp/tests/fixtures/canonical/` (15 файлов), побайтовое сравнение — `d01_d12_match_handwritten_fixtures`, F-04 (D) |

Результаты проверок — TEST_RESULTS.md TR-0008 (ревизия 1), TR-0009 (независимое ревью ревизии 1), TR-0010 (ревизия 2), TR-0011 (независимое ревью ревизии 2, F-05), TR-0012 (ревизия 3).
