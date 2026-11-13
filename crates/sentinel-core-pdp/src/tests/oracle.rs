//! Свойственные тесты: сверка PDP с независимым оракулом (TC-CORE-16/-18 в части модели).
//!
//! Оракул написан отдельно от алгоритма: он не использует `eval`, `worlds`, `precedence`
//! и `conflict_pairs`, а перебирает КОНКРЕТНЫЕ количества совпадений (0..=found+3) по каждому
//! детектору, оценивает условия своим двузначным вычислителем и применяет своё правило
//! подавления: применимая пара «не-BLOCK выше BLOCK», обе истинны в мире, без действительного
//! утверждения, покрывающего контекст.
//!
//! Сверяются: verdict, required_action, action_certainty, fallback, decision_basis,
//! множество possible_winners и отсутствие ложного BLOCK. Генерация случайна, но
//! воспроизводима (xorshift с фиксированным зерном); тест не зависит от внешних crates.
//!
//! Области (CHG7-R1): правила получают случайные области по группам (`hr`, `finance`, обе,
//! без ограничения) и устройствам (`d1`, `d2`, без ограничения); контекст — устройство `d1` и
//! случайное подмножество групп, включая членство в обеих. Оракул сам решает, применимо ли
//! правило, доказуемо ли непересечение пары (только по устройствам — однозначному измерению)
//! и выразимо ли ограниченное утверждение для пары (группы утверждения должны лежать в
//! пересечении групп пары). Бандлы с невыразимым утверждением оракул ожидает отклонёнными.
//!
//! Ограничения оракула (этап 1): классификация CONFIRMED (ось классов не перебирается —
//! проверена golden-тестами G-20…G-22); исключения анализа частей не генерируются;
//! подразделения не генерируются (та же семантика, что у групп).

use super::*;
use crate::policy::Atom;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    fn chance(&mut self, pct: u64) -> bool {
        self.below(100) < pct
    }
}

const DETS: [&str; 2] = ["card", "kw"];
const EXTRA: u32 = 3; // запас сверх найденного, покрывающий все пороги (n ≤ 3, порог детектора 1)

// ---------- Собственное представление условий оракула ----------

#[derive(Clone)]
enum OAtom {
    Count(usize, u32),
    Met(usize),
    Channel(&'static str),
    Incomplete(Option<ReasonCode>),
}

#[derive(Clone)]
enum OExpr {
    Atom(OAtom),
    Or(Box<OExpr>, Box<OExpr>),
    Not(Box<OExpr>),
}

#[derive(Clone)]
struct ORule {
    id: String,
    priority: u32,
    action: Action,
    on_ind: Fallback,
    conjuncts: Vec<OExpr>,
    /// Группы области; пусто — все.
    groups: Vec<&'static str>,
    /// Устройство области; `None` — все.
    device: Option<&'static str>,
}

/// Утверждение конфликтной пары в представлении оракула.
#[derive(Clone, Debug)]
struct OApproval {
    higher: usize,
    lower: usize,
    id: String,
    valid: bool,
    groups: Vec<&'static str>,
    device: Option<&'static str>,
}

struct Case {
    rules: Vec<ORule>,
    complete: bool,
    reasons: Vec<ReasonCode>,
    found: [u32; 2],
    approvals: Vec<OApproval>,
    /// Группы субъекта контекста (устройство контекста всегда `d1`).
    ctx_groups: Vec<&'static str>,
    /// Пары, для которых утверждение невыразимо: бандл должен быть отклонён.
    unapprovable: Vec<(usize, usize)>,
}

const GROUPS: [&str; 2] = ["hr", "finance"];
const CTX_DEVICE: &str = "d1";

fn gen_groups(rng: &mut Rng) -> Vec<&'static str> {
    match rng.below(4) {
        0 => vec![],
        1 => vec!["hr"],
        2 => vec!["finance"],
        _ => vec!["hr", "finance"],
    }
}

fn gen_device(rng: &mut Rng) -> Option<&'static str> {
    match rng.below(3) {
        0 => None,
        1 => Some("d1"),
        _ => Some("d2"),
    }
}

/// Применимо ли правило к контексту (собственная проверка оракула).
fn o_applicable(r: &ORule, ctx_groups: &[&str]) -> bool {
    (r.groups.is_empty() || r.groups.iter().any(|g| ctx_groups.contains(g)))
        && r.device.is_none_or(|d| d == CTX_DEVICE)
}

/// Покрывает ли утверждение контекст.
fn o_covers(a: &OApproval, ctx_groups: &[&str]) -> bool {
    (a.groups.is_empty() || a.groups.iter().any(|g| ctx_groups.contains(g)))
        && a.device.is_none_or(|d| d == CTX_DEVICE)
}

/// Константы атомов канала среди конъюнктов верхнего уровня правила.
fn o_top_channels(r: &ORule) -> Vec<&'static str> {
    r.conjuncts
        .iter()
        .filter_map(|e| if let OExpr::Atom(OAtom::Channel(c)) = e { Some(*c) } else { None })
        .collect()
}

/// Доказуемое непересечение по оракулу: только по однозначным измерениям — устройство области
/// или константа канала среди конъюнктов верхнего уровня (контекст несёт ровно один канал).
fn o_disjoint(h: &ORule, l: &ORule) -> bool {
    if matches!((h.device, l.device), (Some(a), Some(b)) if a != b) {
        return true;
    }
    let (hc, lc) = (o_top_channels(h), o_top_channels(l));
    !hc.is_empty() && !lc.is_empty() && !hc.iter().any(|c| lc.contains(c))
}

/// Пересечение измерения пары: `None` — не ограничено ни одной из областей.
fn meet<T: PartialEq + Copy>(a: &[T], b: &[T]) -> Option<Vec<T>> {
    match (a.is_empty(), b.is_empty()) {
        (true, true) => None,
        (false, true) => Some(a.to_vec()),
        (true, false) => Some(b.to_vec()),
        (false, false) => Some(a.iter().copied().filter(|x| b.contains(x)).collect()),
    }
}

fn strictness(a: Action) -> u8 {
    match a {
        Action::Monitor => 0,
        Action::Alert => 1,
        Action::Block => 2,
    }
}

fn gen_leaf(rng: &mut Rng) -> OExpr {
    match rng.below(3) {
        0 => OExpr::Atom(OAtom::Count(rng.below(2) as usize, 1 + rng.below(3) as u32)),
        1 => OExpr::Atom(OAtom::Met(rng.below(2) as usize)),
        _ => OExpr::Atom(OAtom::Channel(if rng.chance(70) { "USB" } else { "PRINT" })),
    }
}

fn gen_conjunct(rng: &mut Rng) -> OExpr {
    match rng.below(4) {
        0 => OExpr::Or(Box::new(gen_leaf(rng)), Box::new(gen_leaf(rng))),
        1 => OExpr::Not(Box::new(gen_leaf(rng))),
        _ => gen_leaf(rng),
    }
}

fn gen_case(rng: &mut Rng) -> Case {
    let n = 1 + rng.below(5) as usize;
    let mut rules = Vec::new();
    for i in 0..n {
        let mut conjuncts: Vec<OExpr> = (0..1 + rng.below(2)).map(|_| gen_conjunct(rng)).collect();
        if rng.chance(30) {
            let reason = match rng.below(3) {
                0 => None,
                1 => Some(ReasonCode::Timeout),
                _ => Some(ReasonCode::UnsupportedFormat),
            };
            conjuncts.push(OExpr::Atom(OAtom::Incomplete(reason)));
        }
        let action = match rng.below(3) {
            0 => Action::Monitor,
            1 => Action::Alert,
            _ => Action::Block,
        };
        rules.push(ORule {
            id: format!("R{i:02}"),
            priority: 1 + rng.below(3) as u32, // намеренно с повторами
            action,
            on_ind: if rng.chance(50) { Fallback::Block } else { Fallback::AllowWithAudit },
            conjuncts,
            groups: gen_groups(rng),
            device: gen_device(rng),
        });
    }
    // Порядок ≻ по собственному компаратору оракула.
    rules.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then(strictness(b.action).cmp(&strictness(a.action)))
            .then(a.id.cmp(&b.id))
    });
    let complete = rng.chance(40);
    let reasons = if complete {
        vec![]
    } else {
        match rng.below(3) {
            0 => vec![ReasonCode::Timeout],
            1 => vec![ReasonCode::UnsupportedFormat],
            _ => vec![ReasonCode::Timeout, ReasonCode::UnsupportedFormat],
        }
    };
    let found = [rng.below(3) as u32, rng.below(3) as u32];
    let ctx_groups = gen_groups(rng);
    let mut approvals = Vec::new();
    let mut unapprovable = Vec::new();
    let mut k = 0;
    for h in 0..rules.len() {
        for l in h + 1..rules.len() {
            if rules[l].action != Action::Block
                || rules[h].action == Action::Block
                || o_disjoint(&rules[h], &rules[l])
            {
                continue;
            }
            // Утверждение должно лежать в пересечении областей пары и быть ограниченным.
            let groups = meet(&rules[h].groups, &rules[l].groups);
            let dev_h: Vec<&str> = rules[h].device.into_iter().collect();
            let dev_l: Vec<&str> = rules[l].device.into_iter().collect();
            let devices = meet(&dev_h, &dev_l);
            if groups.as_ref().is_some_and(|g| g.is_empty()) || devices.as_ref().is_some_and(|d| d.is_empty())
            {
                unapprovable.push((h, l));
                continue;
            }
            let device = devices.map(|d| d[0]);
            // Ограниченность: если ни группы, ни устройство не заданы парой — ограничиваем группами
            // контекстно-нейтрально (обе группы), чтобы утверждение осталось в пересечении.
            let groups = match groups {
                Some(g) => g,
                None if device.is_some() => vec![],
                None => GROUPS.to_vec(),
            };
            approvals.push(OApproval {
                higher: h,
                lower: l,
                id: format!("A{k}"),
                valid: rng.chance(50),
                groups,
                device,
            });
            k += 1;
        }
    }
    Case { rules, complete, reasons, found, approvals, ctx_groups, unapprovable }
}

// ---------- Перевод случая во вход PDP ----------

fn to_expr(e: &OExpr) -> Expr {
    match e {
        OExpr::Atom(a) => Expr::Atom(match a {
            OAtom::Count(d, n) => Atom::CountAtLeast(DetectorId::new(DETS[*d]), *n),
            OAtom::Met(d) => Atom::ThresholdMet(DetectorId::new(DETS[*d])),
            OAtom::Channel(c) => Atom::ChannelIs(Channel::new(*c)),
            OAtom::Incomplete(r) => Atom::InspectionIncomplete(*r),
        }),
        OExpr::Or(a, b) => Expr::Or(vec![to_expr(a), to_expr(b)]),
        OExpr::Not(a) => Expr::Not(Box::new(to_expr(a))),
    }
}

fn to_scope(groups: &[&str], device: Option<&str>) -> Scope {
    Scope {
        groups: groups.iter().map(|g| GroupId::new(*g)).collect(),
        devices: device.into_iter().map(DeviceRef::new).collect(),
        ..Scope::default()
    }
}

fn to_ir(c: &Case) -> PolicyIr {
    let rules = c
        .rules
        .iter()
        .map(|r| {
            let cond = if r.conjuncts.len() == 1 {
                to_expr(&r.conjuncts[0])
            } else {
                Expr::And(r.conjuncts.iter().map(to_expr).collect())
            };
            let mut x = rule(&r.id, r.priority, r.action, cond);
            x.on_indeterminate = r.on_ind;
            x.scope = to_scope(&r.groups, r.device);
            x
        })
        .collect();
    let mut p = ir(rules);
    p.approvals = c
        .approvals
        .iter()
        .map(|a| {
            let mut x = pca(&a.id, &c.rules[a.higher].id, &c.rules[a.lower].id);
            x.scope = to_scope(&a.groups, a.device);
            x
        })
        .collect();
    p
}

fn to_ctx(c: &Case) -> DecisionContext {
    DecisionContext {
        scope_attrs: c.ctx_groups.iter().map(|g| ScopeAttr::Group(GroupId::new(*g))).collect(),
        device_ref: DeviceRef::new(CTX_DEVICE),
        ..ctx()
    }
}

fn to_insp(c: &Case) -> InspectionResult {
    let any = c.found.iter().any(|f| *f >= 1);
    let status = match (any, c.complete) {
        (true, _) => InspectionStatus::Match,
        (false, true) => InspectionStatus::NoMatch,
        (false, false) => InspectionStatus::NotInspected,
    };
    insp(
        status,
        &c.reasons,
        if c.complete { TextCoverage::Complete } else { TextCoverage::Partial },
        StructureEnumeration::Complete,
        Classification::Confirmed,
        &[],
        DETS.iter().enumerate().map(|(i, d)| det(d, c.found[i], c.found[i] >= 1)).collect(),
        "TXT",
    )
}

// ---------- Оракул ----------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Outcome {
    Block,
    Allow(Option<Action>),
}

fn o_eval(e: &OExpr, c: &Case, counts: [u32; 2]) -> bool {
    match e {
        OExpr::Atom(OAtom::Count(d, n)) => counts[*d] >= *n,
        OExpr::Atom(OAtom::Met(d)) => counts[*d] >= 1,
        OExpr::Atom(OAtom::Channel(ch)) => *ch == "USB",
        OExpr::Atom(OAtom::Incomplete(None)) => !c.complete,
        OExpr::Atom(OAtom::Incomplete(Some(r))) => !c.complete && c.reasons.contains(r),
        OExpr::Or(a, b) => o_eval(a, c, counts) || o_eval(b, c, counts),
        OExpr::Not(a) => !o_eval(a, c, counts),
    }
}

/// Победитель среди ПРИМЕНИМЫХ правил (`truth` уже учитывает применимость).
fn o_winner(c: &Case, truth: &[bool], suppress: bool) -> Option<usize> {
    (0..c.rules.len()).find(|&h| {
        if !truth[h] {
            return false;
        }
        if !suppress {
            return true;
        }
        // Подавление: ниже есть истинное применимое BLOCK-правило, и для пары нет действительного
        // утверждения, покрывающего контекст.
        !(h + 1..c.rules.len()).any(|l| {
            truth[l]
                && c.rules[l].action == Action::Block
                && c.rules[h].action != Action::Block
                && !c
                    .approvals
                    .iter()
                    .any(|a| a.higher == h && a.lower == l && a.valid && o_covers(a, &c.ctx_groups))
        })
    })
}

struct Expected {
    verdict: Verdict,
    required: Option<Action>,
    certainty: Option<ActionCertainty>,
    fallback: Option<Fallback>,
    basis: DecisionBasis,
    possible_winners: Vec<String>,
}

fn oracle(c: &Case) -> Expected {
    let max = |d: usize| if c.complete { c.found[d] } else { c.found[d] + EXTRA };
    let mut outcomes: Vec<Outcome> = Vec::new();
    let mut winners_sup: Vec<Option<usize>> = Vec::new();
    let mut winners_all: Vec<usize> = Vec::new();
    for c0 in c.found[0]..=max(0) {
        for c1 in c.found[1]..=max(1) {
            let counts = [c0, c1];
            let truth: Vec<bool> = c
                .rules
                .iter()
                .map(|r| o_applicable(r, &c.ctx_groups) && r.conjuncts.iter().all(|e| o_eval(e, c, counts)))
                .collect();
            let w = o_winner(c, &truth, true);
            let out = match w {
                Some(i) if c.rules[i].action == Action::Block => Outcome::Block,
                Some(i) => Outcome::Allow(Some(c.rules[i].action)),
                None if c.complete => Outcome::Allow(None),
                None => Outcome::Allow(Some(Action::Monitor)), // IIA-default
            };
            if !outcomes.contains(&out) {
                outcomes.push(out);
            }
            if !winners_sup.contains(&w) {
                winners_sup.push(w);
            }
            if let Some(u) = o_winner(c, &truth, false) {
                if !winners_all.contains(&u) {
                    winners_all.push(u);
                }
            }
        }
    }
    let all_block = outcomes.iter().all(|o| *o == Outcome::Block);
    let all_allow = outcomes.iter().all(|o| matches!(o, Outcome::Allow(_)));
    let no_winner = winners_sup.contains(&None);
    let mut rule_winners: Vec<usize> = winners_sup.iter().flatten().copied().collect();
    rule_winners.sort_unstable();
    let mut pw: Vec<String> = rule_winners.iter().map(|&i| c.rules[i].id.clone()).collect();
    pw.sort();
    let single = winners_sup.len() == 1;
    let possible_winners = if single { vec![] } else { pw };
    if all_block {
        Expected {
            verdict: Verdict::Block,
            required: Some(Action::Block),
            certainty: None,
            fallback: None,
            basis: DecisionBasis::RuleMatched,
            possible_winners,
        }
    } else if all_allow {
        if outcomes == [Outcome::Allow(None)] {
            return Expected {
                verdict: Verdict::Allow,
                required: None,
                certainty: None,
                fallback: None,
                basis: DecisionBasis::NoRuleMatched,
                possible_winners,
            };
        }
        let acts: Vec<Action> =
            outcomes.iter().filter_map(|o| if let Outcome::Allow(a) = o { *a } else { None }).collect();
        let strictest = acts.iter().copied().max_by_key(|a| strictness(*a));
        let mut levels: Vec<u8> = acts.iter().map(|a| strictness(*a)).collect();
        levels.sort_unstable();
        levels.dedup();
        Expected {
            verdict: Verdict::Allow,
            required: strictest,
            certainty: Some(if levels.len() == 1 {
                ActionCertainty::Definite
            } else {
                ActionCertainty::Possible
            }),
            fallback: None,
            basis: if no_winner {
                DecisionBasis::OutcomeIndependentOfUnknown
            } else {
                DecisionBasis::RuleMatched
            },
            possible_winners,
        }
    } else {
        let mut contributors = rule_winners.clone();
        for u in &winners_all {
            if !contributors.contains(u) {
                contributors.push(*u);
            }
        }
        let fb = if contributors.iter().any(|&i| c.rules[i].on_ind == Fallback::Block) {
            Fallback::Block
        } else {
            Fallback::AllowWithAudit
        };
        Expected {
            verdict: Verdict::Indeterminate,
            required: Some(if fb == Fallback::Block { Action::Block } else { Action::Monitor }),
            certainty: None,
            fallback: Some(fb),
            basis: DecisionBasis::Indeterminate,
            possible_winners,
        }
    }
}

fn dv_for(c: &Case) -> DeviationValidity {
    let ids: Vec<&str> = c.approvals.iter().filter(|a| a.valid).map(|a| a.id.as_str()).collect();
    valid(&ids)
}

#[test]
fn pdp_agrees_with_brute_force_oracle() {
    let mut rng = Rng(0x5EED_0000_CAFE_F00D);
    let mut stats = [0usize; 3];
    let mut rejected = 0usize;
    let mut multi_membership = 0usize;
    for case_no in 0..4000 {
        let c = gen_case(&mut rng);
        let what = format!(
            "случай {case_no}: правила {:?}, ctx_groups={:?}, complete={}, reasons={:?}, found={:?}, approvals={:?}, unapprovable={:?}",
            c.rules
                .iter()
                .map(|r| format!("{}:{}:{:?}:{:?}:g{:?}:d{:?}", r.id, r.priority, r.action, r.on_ind, r.groups, r.device))
                .collect::<Vec<_>>(),
            c.ctx_groups,
            c.complete,
            c.reasons,
            c.found,
            c.approvals,
            c.unapprovable
        );
        // Проверка загрузки: бандл с невыразимым утверждением отклоняется, остальные принимаются.
        let v = validate(to_ir(&c));
        if !c.unapprovable.is_empty() {
            match v {
                Err(BundleError::UnapprovedPrecedenceConflict { higher, lower, .. }) => {
                    assert!(
                        c.unapprovable.iter().any(
                            |&(h, l)| c.rules[h].id == higher.as_str() && c.rules[l].id == lower.as_str()
                        ),
                        "отклонена не та пара; {what}"
                    );
                }
                other => panic!("ожидалось отклонение конфликтной пары, получено {other:?}; {what}"),
            }
            rejected += 1;
            continue;
        }
        let p = CompiledPolicy::unsigned_for_tests(
            v.unwrap_or_else(|e| panic!("бандл должен пройти проверку: {e:?}; {what}")),
        );
        if c.ctx_groups.len() == 2 {
            multi_membership += 1;
        }
        let i = to_insp(&c);
        let dv = dv_for(&c);
        let cx = to_ctx(&c);
        let d = crate::Pdp::decide(&p, &cx, &i, &dv).expect("решение должно быть получено");
        let e = oracle(&c);
        assert_eq!(d.verdict, e.verdict, "verdict; {what}; explanation={:?}", d.explanation);
        assert_eq!(d.required_action, e.required, "required_action; {what}");
        assert_eq!(d.action_certainty, e.certainty, "action_certainty; {what}");
        assert_eq!(d.fallback, e.fallback, "fallback; {what}");
        assert_eq!(d.decision_basis, e.basis, "decision_basis; {what}");
        let pw: Vec<String> = d.possible_winners.iter().map(|r| r.as_str().to_string()).collect();
        assert_eq!(pw, e.possible_winners, "possible_winners; {what}");
        // Отсутствие ложного BLOCK: BLOCK только если оракул не нашёл ни одного мира без BLOCK.
        if d.verdict == Verdict::Block {
            assert_eq!(e.verdict, Verdict::Block, "ложный BLOCK; {what}");
        }
        // Детерминизм на том же входе.
        assert_eq!(crate::Pdp::decide(&p, &cx, &i, &dv).unwrap(), d, "детерминизм; {what}");
        stats[match d.verdict {
            Verdict::Block => 0,
            Verdict::Allow => 1,
            Verdict::Indeterminate => 2,
        }] += 1;
    }
    // Генератор должен давать все три вердикта, отклонённые бандлы и множественное членство,
    // иначе сверка вырождена.
    assert!(stats.iter().all(|s| *s > 50), "распределение вердиктов {stats:?}");
    assert!(
        rejected > 100 && multi_membership > 100,
        "отклонено {rejected}, членство в обеих группах {multi_membership}"
    );
}
