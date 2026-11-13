//! Модель политики — промежуточное представление (IR) бандла (SENTINEL_CORE_DESIGN.md §6.1).
//!
//! IR поступает из подписанного бандла. Проверка подписи, формат бандла на диске и загрузчики
//! `PolicyBundle::load_*` на этапе 1 не реализованы (вопросы O-05, O-06). Здесь — только
//! содержательная модель и функции, которые нужны и загрузке (§6.7), и решению (§6.3).

use sentinel_core_types::{
    Action, Channel, DecisionContext, DetectorId, DeviceRef, Fallback, FileClass, GroupId, Operation,
    OrgUnitId, ReasonCode, RecordId, RuleId, ScopeAttr,
};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;

/// Поддерживаемая major-версия формата IR (§11).
pub const SUPPORTED_FORMAT_MAJOR: u32 = 1;

/// Область применения. Пустое измерение означает «все» (§6.1).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Scope {
    pub channels: Vec<Channel>,
    pub operations: Vec<Operation>,
    pub groups: Vec<GroupId>,
    pub org_units: Vec<OrgUnitId>,
    pub devices: Vec<DeviceRef>,
}

impl Scope {
    /// Совпадает ли область с контекстом решения: каждое непустое измерение содержит значение
    /// контекста (для групп и подразделений — хотя бы одно из атрибутов субъекта).
    pub fn matches(&self, ctx: &DecisionContext) -> bool {
        let groups_ok = self.groups.is_empty()
            || ctx.scope_attrs.iter().any(|a| matches!(a, ScopeAttr::Group(g) if self.groups.contains(g)));
        let units_ok = self.org_units.is_empty()
            || ctx
                .scope_attrs
                .iter()
                .any(|a| matches!(a, ScopeAttr::OrgUnit(u) if self.org_units.contains(u)));
        (self.channels.is_empty() || self.channels.contains(&ctx.channel))
            && (self.operations.is_empty() || self.operations.contains(&ctx.operation))
            && groups_ok
            && units_ok
            && (self.devices.is_empty() || self.devices.contains(&ctx.device_ref))
    }

    /// Неограниченная область: все измерения пусты.
    pub fn is_unbounded(&self) -> bool {
        self.channels.is_empty()
            && self.operations.is_empty()
            && self.groups.is_empty()
            && self.org_units.is_empty()
            && self.devices.is_empty()
    }

    /// Ограниченная область записи-отступления: не «все пользователи всех устройств» (§4.2).
    /// Толкование этапа 1: задано хотя бы одно из измерений субъекта или устройства
    /// (группы, подразделения, устройства). Ограничение только по каналу или операции
    /// оставляет отступление действующим для всех пользователей.
    pub fn is_bounded_subject(&self) -> bool {
        !(self.groups.is_empty() && self.org_units.is_empty() && self.devices.is_empty())
    }

    /// Доказуемо пустое пересечение областей (§6.7): в каком-либо **однозначном** измерении
    /// контекста (канал, операция, устройство) обе области заданы и не имеют общих значений.
    ///
    /// Группы и подразделения доказательства не дают (CHG7-R1): субъект может состоять в
    /// нескольких группах и подразделениях одновременно (`DecisionContext::scope_attrs` —
    /// список), поэтому области `groups = [hr]` и `groups = [finance]` пересекаются на
    /// субъекте, входящем в обе группы.
    pub fn provably_disjoint(&self, other: &Scope) -> bool {
        fn disjoint<T: PartialEq>(a: &[T], b: &[T]) -> bool {
            !a.is_empty() && !b.is_empty() && !a.iter().any(|x| b.contains(x))
        }
        disjoint(&self.channels, &other.channels)
            || disjoint(&self.operations, &other.operations)
            || disjoint(&self.devices, &other.devices)
    }

    /// Лежит ли эта область внутри пересечения областей `a` и `b` (§6.7, утверждение
    /// конфликта): каждый контекст, совпадающий с этой областью, совпадает и с `a`, и с `b`.
    /// Измерение, ограниченное хотя бы одной из областей пары, должно быть задано и в этой
    /// области и содержать только значения из пересечения. Для групп и подразделений это
    /// единственное достаточное условие при многозначном членстве: контекст может нести
    /// ровно одну группу из этой области, и она обязана входить в обе области пары. Следствие:
    /// пара `groups = [hr]` / `groups = [finance]` пересекается (субъект в обеих группах), но
    /// ограниченное утверждение для неё невыразимо — такой бандл отклоняется до переработки
    /// правил (CHG7-R1).
    pub fn within_intersection(&self, a: &Scope, b: &Scope) -> bool {
        fn dim<T: PartialEq + Clone>(own: &[T], a: &[T], b: &[T]) -> bool {
            let allowed: Option<Vec<T>> = match (a.is_empty(), b.is_empty()) {
                (true, true) => None,
                (false, true) => Some(a.to_vec()),
                (true, false) => Some(b.to_vec()),
                (false, false) => Some(a.iter().filter(|x| b.contains(x)).cloned().collect()),
            };
            match allowed {
                None => true,
                Some(set) => !own.is_empty() && own.iter().all(|x| set.contains(x)),
            }
        }
        dim(&self.channels, &a.channels, &b.channels)
            && dim(&self.operations, &a.operations, &b.operations)
            && dim(&self.groups, &a.groups, &b.groups)
            && dim(&self.org_units, &a.org_units, &b.org_units)
            && dim(&self.devices, &a.devices, &b.devices)
    }
}

/// Детектор набора бандла. Порог `threshold` — в единицах количества совпадений, как в
/// §6.3, шаг 6 («порог детектора для `threshold_met`» среди порогов количества). Для словарей
/// с весами, где порог задаётся по `score`, этот контракт не определён (отчёт этапа 1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetectorSpec {
    pub detector_id: DetectorId,
    pub threshold: u32,
}

/// Атом условия (§6.1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Atom {
    /// `detector(id).threshold_met`
    ThresholdMet(DetectorId),
    /// `detector(id).count >= n`
    CountAtLeast(DetectorId, u32),
    /// `inspection.incomplete[(reason)]`
    InspectionIncomplete(Option<ReasonCode>),
    /// `file.class = X`
    FileClassIs(FileClass),
    /// `context.channel = X`
    ChannelIs(Channel),
    /// `context.operation = X`
    OperationIs(Operation),
}

impl Atom {
    /// Атом по содержимому (монотонен при неполном покрытии, §4.1).
    pub fn is_content(&self) -> bool {
        matches!(self, Atom::ThresholdMet(_) | Atom::CountAtLeast(..))
    }

    /// Детектор атома по содержимому.
    pub fn detector(&self) -> Option<&DetectorId> {
        match self {
            Atom::ThresholdMet(d) | Atom::CountAtLeast(d, _) => Some(d),
            _ => None,
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::ThresholdMet(d) => write!(f, "detector({d}).threshold_met"),
            Atom::CountAtLeast(d, n) => write!(f, "detector({d}).count >= {n}"),
            Atom::InspectionIncomplete(None) => f.write_str("inspection.incomplete"),
            Atom::InspectionIncomplete(Some(r)) => write!(f, "inspection.incomplete({})", r.as_str()),
            Atom::FileClassIs(c) => write!(f, "file.class = {c}"),
            Atom::ChannelIs(c) => write!(f, "context.channel = {c}"),
            Atom::OperationIs(o) => write!(f, "context.operation = {o}"),
        }
    }
}

/// Условие правила: AND, OR, NOT над атомами (§6.1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Atom(Atom),
    And(Vec<Expr>),
    Or(Vec<Expr>),
    Not(Box<Expr>),
}

impl Expr {
    /// Обход всех атомов.
    pub fn for_each_atom<'a>(&'a self, f: &mut impl FnMut(&'a Atom)) {
        match self {
            Expr::Atom(a) => f(a),
            Expr::And(xs) | Expr::Or(xs) => xs.iter().for_each(|x| x.for_each_atom(f)),
            Expr::Not(x) => x.for_each_atom(f),
        }
    }

    /// Конъюнкты верхнего уровня: элементы `AND` верхнего уровня или само выражение.
    pub fn top_conjuncts(&self) -> Vec<&Expr> {
        match self {
            Expr::And(xs) => xs.iter().collect(),
            other => vec![other],
        }
    }

    /// Содержит ли условие атом по содержимому или `file.class` (класс (a) конфликта, §6.7).
    pub fn has_content_or_class_atom(&self) -> bool {
        let mut found = false;
        self.for_each_atom(&mut |a| found |= a.is_content() || matches!(a, Atom::FileClassIs(_)));
        found
    }

    /// Есть ли атом `inspection.incomplete` среди конъюнктов верхнего уровня.
    pub fn guards_incompleteness(&self) -> bool {
        self.top_conjuncts().iter().any(|c| matches!(c, Expr::Atom(Atom::InspectionIncomplete(_))))
    }
}

/// Правило (§6.1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    pub rule_id: RuleId,
    pub priority: u32,
    pub protected_operation: bool,
    pub scope: Scope,
    pub condition: Expr,
    pub action: Action,
    /// ADR-0019: исходное действие и исключение понижения, если оно применено для платформы.
    /// Проверка исключений требует манифеста возможностей (режим `EnforcementReady`) и на
    /// этапе 1 не выполняется; поля только переносятся в решение.
    pub requested_action: Option<Action>,
    pub capability_exception_id: Option<RecordId>,
    pub on_indeterminate: Fallback,
    pub data_level: sentinel_core_types::DataLevel,
}

/// Порядок ≻ (§6.3, шаг 2): больший приоритет; при равенстве — более строгое действие; при
/// равенстве — меньший `rule_id` в байтовом порядке. `Less` означает «выше по порядку».
pub fn precedence(a: &Rule, b: &Rule) -> Ordering {
    b.priority
        .cmp(&a.priority)
        .then(b.action.strictness().cmp(&a.action.strictness()))
        .then(a.rule_id.cmp(&b.rule_id))
}

/// Контракт неполной инспекции (§6.1, §6.3, шаг 7). Действие — только MONITOR или ALERT.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncompleteInspectionAction {
    pub record_id: RecordId,
    pub scope: Scope,
    pub action: IncompleteAction,
}

/// Действие контракта неполной инспекции.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncompleteAction {
    Monitor,
    Alert,
}

impl IncompleteAction {
    /// Как действие решения.
    pub fn as_action(self) -> Action {
        match self {
            IncompleteAction::Monitor => Action::Monitor,
            IncompleteAction::Alert => Action::Alert,
        }
    }
}

/// Вид части, который может исключить `PartInspectionExclusion` (без UNKNOWN, §4.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExcludableKind {
    Image,
    EmbeddedObject,
    Attachment,
}

/// Реквизиты согласования (ADR-0019 §2) и срок действия (§6.8).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Approval {
    pub justification: String,
    pub author: String,
    pub approver: String,
    pub approved_at: i64,
    pub expires_at: Option<i64>,
    /// Предельная длительность в офлайне, секунды. Ядро не вычисляет действительность по
    /// времени (это делает хост, INV-H3); поле входит в контракт бандла.
    pub max_offline_validity: u64,
}

/// Исключение анализа частей объекта (§4.2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartInspectionExclusion {
    pub exclusion_id: RecordId,
    pub scope: Scope,
    pub kinds: Vec<ExcludableKind>,
    pub approval: Approval,
}

/// Утверждение конфликта приоритетов (§6.1, §6.7).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrecedenceConflictApproval {
    pub approval_id: RecordId,
    pub higher_rule_id: RuleId,
    pub lower_rule_id: RuleId,
    pub scope: Scope,
    pub approval: Approval,
}

/// IR бандла (§6.1). Подписанные метаданные бандла (`platform` и др.) проверяются при загрузке
/// режима `EnforcementReady`; на этапе 1 `platform` только переносится.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyIr {
    pub bundle_id: sentinel_core_types::BundleId,
    pub policy_version: u64,
    pub format_version: (u32, u32),
    pub platform: String,
    pub detector_set: Vec<DetectorSpec>,
    pub rules: Vec<Rule>,
    pub incomplete_actions: Vec<IncompleteInspectionAction>,
    pub exclusions: Vec<PartInspectionExclusion>,
    pub approvals: Vec<PrecedenceConflictApproval>,
}

/// Класс конфликта приоритетов (§6.7) — для отчёта и объяснения.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConflictClass {
    /// Условие h содержит атомы по содержимому или `file.class`, `h.on_indeterminate = ALLOW_WITH_AUDIT`.
    A,
    /// Условие l содержит атом `inspection.incomplete` конъюнктом верхнего уровня.
    B,
    /// Все остальные, в том числе затенение только по контексту.
    C,
}

/// Константы атомов `context.channel`, `context.operation`, `file.class` среди конъюнктов
/// верхнего уровня.
fn top_level_constants(e: &Expr) -> (BTreeSet<&Channel>, BTreeSet<&Operation>, BTreeSet<&FileClass>) {
    let (mut ch, mut op, mut cl) = (BTreeSet::new(), BTreeSet::new(), BTreeSet::new());
    for c in e.top_conjuncts() {
        match c {
            Expr::Atom(Atom::ChannelIs(x)) => {
                ch.insert(x);
            }
            Expr::Atom(Atom::OperationIs(x)) => {
                op.insert(x);
            }
            Expr::Atom(Atom::FileClassIs(x)) => {
                cl.insert(x);
            }
            _ => {}
        }
    }
    (ch, op, cl)
}

/// Доказуемое непересечение областей двух правил (§6.7): пустое пересечение в измерении
/// области или разные константы одного атома среди конъюнктов верхнего уровня. Класс
/// конкретного объекта для доказательства не используется (INV-C14).
pub fn rules_provably_disjoint(h: &Rule, l: &Rule) -> bool {
    if h.scope.provably_disjoint(&l.scope) {
        return true;
    }
    let (hc, ho, hf) = top_level_constants(&h.condition);
    let (lc, lo, lf) = top_level_constants(&l.condition);
    fn differ<T: Ord>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> bool {
        !a.is_empty() && !b.is_empty() && a.is_disjoint(b)
    }
    differ(&hc, &lc) || differ(&ho, &lo) || differ(&hf, &lf)
}

/// Конфликтная пара: индексы правил в переданном срезе (h выше l по ≻) и класс.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConflictPair {
    pub higher: usize,
    pub lower: usize,
    pub class: ConflictClass,
}

/// Все конфликтные пары среди правил, упорядоченных по ≻ (§6.7, CHG5-AR7-02): h ≻ l, действие
/// l — BLOCK, действие h — MONITOR или ALERT, области доказуемо не разделены.
pub fn conflict_pairs(ordered_rules: &[&Rule]) -> Vec<ConflictPair> {
    let mut out = Vec::new();
    for (i, h) in ordered_rules.iter().enumerate() {
        if h.action == Action::Block {
            continue;
        }
        for (j, l) in ordered_rules.iter().enumerate().skip(i + 1) {
            if l.action != Action::Block || rules_provably_disjoint(h, l) {
                continue;
            }
            let class = if h.condition.has_content_or_class_atom()
                && h.on_indeterminate == Fallback::AllowWithAudit
            {
                ConflictClass::A
            } else if l.condition.guards_incompleteness() {
                ConflictClass::B
            } else {
                ConflictClass::C
            };
            out.push(ConflictPair { higher: i, lower: j, class });
        }
    }
    out
}
