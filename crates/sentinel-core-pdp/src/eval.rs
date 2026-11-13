//! Значения атомов и условий (SENTINEL_CORE_DESIGN.md §6.3, шаги 3 и 6).

use crate::policy::{Atom, DetectorSpec, Expr, Rule};
use sentinel_core_types::{Classification, DecisionContext, DetectorId, FileClass, InspectionResult, Truth};
use std::collections::{BTreeMap, BTreeSet};

/// Вход вычисления: результат инспекции, контекст и режим атомов по содержимому.
pub(crate) struct Inputs<'a> {
    pub insp: &'a InspectionResult,
    pub ctx: &'a DecisionContext,
    pub detectors: &'a [DetectorSpec],
    /// Атомы по содержимому двузначны: полное покрытие или применённое исключение (§6.3, шаг 3).
    pub two_valued: bool,
    pub complete: bool,
}

impl Inputs<'_> {
    fn threshold(&self, d: &DetectorId) -> u32 {
        // Наличие детектора в наборе проверено при загрузке (§6.7, «Состав»).
        self.detectors.iter().find(|s| &s.detector_id == d).map_or(u32::MAX, |s| s.threshold)
    }

    fn found_met(&self, d: &DetectorId) -> bool {
        self.insp.detection(d).is_some_and(|r| r.threshold_met)
    }

    fn ambiguous(&self) -> bool {
        matches!(self.insp.classification, Classification::Ambiguous(_))
    }

    /// Трёхзначное значение атома без перебора миров.
    pub fn atom(&self, a: &Atom) -> Truth {
        let content = |known_true: bool| {
            if known_true {
                Truth::True
            } else if self.two_valued {
                Truth::False
            } else {
                Truth::Unknown
            }
        };
        match a {
            Atom::ThresholdMet(d) => content(self.found_met(d)),
            Atom::CountAtLeast(d, n) => content(self.insp.found_count(d) >= *n),
            Atom::InspectionIncomplete(reason) => {
                if self.complete {
                    Truth::False
                } else {
                    match reason {
                        None => Truth::True,
                        Some(r) if self.insp.reasons.contains(r) => Truth::True,
                        Some(_) => Truth::False,
                    }
                }
            }
            Atom::FileClassIs(c) => {
                if self.ambiguous() {
                    Truth::Unknown
                } else {
                    truth(&self.insp.file_class == c)
                }
            }
            Atom::ChannelIs(c) => truth(&self.ctx.channel == c),
            Atom::OperationIs(o) => truth(&self.ctx.operation == o),
        }
    }

    /// Трёхзначное значение условия и атом, определивший итог (§6.6).
    pub fn expr(&self, e: &Expr) -> (Truth, Option<String>) {
        match e {
            Expr::Atom(a) => (self.atom(a), Some(a.to_string())),
            Expr::Not(x) => {
                let (v, at) = self.expr(x);
                (not(v), at)
            }
            Expr::And(xs) => {
                let vals: Vec<_> = xs.iter().map(|x| self.expr(x)).collect();
                if let Some((_, at)) = vals.iter().find(|(v, _)| *v == Truth::False) {
                    (Truth::False, at.clone())
                } else if let Some((_, at)) = vals.iter().find(|(v, _)| *v == Truth::Unknown) {
                    (Truth::Unknown, at.clone())
                } else {
                    (Truth::True, if vals.len() == 1 { vals[0].1.clone() } else { None })
                }
            }
            Expr::Or(xs) => {
                let vals: Vec<_> = xs.iter().map(|x| self.expr(x)).collect();
                if let Some((_, at)) = vals.iter().find(|(v, _)| *v == Truth::True) {
                    (Truth::True, at.clone())
                } else if let Some((_, at)) = vals.iter().find(|(v, _)| *v == Truth::Unknown) {
                    (Truth::Unknown, at.clone())
                } else {
                    (Truth::False, if vals.len() == 1 { vals[0].1.clone() } else { None })
                }
            }
        }
    }

    /// Двузначное значение условия в мире.
    pub fn in_world(&self, e: &Expr, w: &World) -> bool {
        match e {
            Expr::Atom(a) => match a {
                Atom::CountAtLeast(d, n) => w.count(d, self) >= *n,
                Atom::ThresholdMet(d) => self.found_met(d) || w.count(d, self) >= self.threshold(d),
                Atom::FileClassIs(c) if self.ambiguous() => w.class.as_ref() == Some(c),
                other => self.atom(other) == Truth::True,
            },
            Expr::Not(x) => !self.in_world(x, w),
            Expr::And(xs) => xs.iter().all(|x| self.in_world(x, w)),
            Expr::Or(xs) => xs.iter().any(|x| self.in_world(x, w)),
        }
    }
}

fn truth(b: bool) -> Truth {
    if b {
        Truth::True
    } else {
        Truth::False
    }
}

fn not(v: Truth) -> Truth {
    match v {
        Truth::True => Truth::False,
        Truth::False => Truth::True,
        Truth::Unknown => Truth::Unknown,
    }
}

/// Мир: фактические количества по детекторам с неизвестными атомами и, при неоднозначной
/// классификации, класс (`None` — значение OTHER).
#[derive(Clone, Debug)]
pub(crate) struct World {
    counts: BTreeMap<DetectorId, u32>,
    pub class: Option<FileClass>,
}

impl World {
    fn count(&self, d: &DetectorId, inputs: &Inputs<'_>) -> u32 {
        self.counts.get(d).copied().unwrap_or_else(|| inputs.insp.found_count(d))
    }
}

/// Перечень миров (§6.3, шаг 6). Для детектора количество не меньше найденного; миры
/// различаются только интервалами между порогами из атомов `count >= n` и порогом детектора
/// для `threshold_met`, превышающими найденное. Представитель интервала — его нижняя граница.
/// При неоднозначной классификации класс — отдельное измерение: каждая константа `file.class`
/// применимых правил и OTHER (§4.3). При двузначных атомах по содержимому мир один.
pub(crate) fn worlds(rules: &[&Rule], inputs: &Inputs<'_>) -> (Vec<World>, Vec<Option<FileClass>>) {
    let mut thresholds: BTreeMap<DetectorId, BTreeSet<u32>> = BTreeMap::new();
    let mut classes: BTreeSet<FileClass> = BTreeSet::new();
    for r in rules {
        r.condition.for_each_atom(&mut |a| match a {
            Atom::CountAtLeast(d, n) => {
                thresholds.entry(d.clone()).or_default().insert(*n);
            }
            Atom::ThresholdMet(d) => {
                thresholds.entry(d.clone()).or_default().insert(inputs.threshold(d));
            }
            Atom::FileClassIs(c) => {
                classes.insert(c.clone());
            }
            _ => {}
        });
    }
    let class_axis: Vec<Option<FileClass>> = if inputs.ambiguous() {
        classes.into_iter().map(Some).chain(std::iter::once(None)).collect()
    } else {
        vec![None]
    };
    let axes: Vec<(DetectorId, Vec<u32>)> = if inputs.two_valued {
        Vec::new()
    } else {
        thresholds
            .into_iter()
            .map(|(d, ns)| {
                let f = inputs.insp.found_count(&d);
                let mut pts = vec![f];
                pts.extend(ns.into_iter().filter(|n| *n > f));
                (d, pts)
            })
            .collect()
    };
    let mut out = Vec::new();
    for class in &class_axis {
        let mut idx = vec![0usize; axes.len()];
        loop {
            let counts = axes.iter().zip(&idx).map(|((d, pts), i)| (d.clone(), pts[*i])).collect();
            out.push(World { counts, class: class.clone() });
            // Следующая комбинация: счётчик с переносом, последний индекс меняется быстрее.
            let mut exhausted = true;
            for k in (0..axes.len()).rev() {
                idx[k] += 1;
                if idx[k] < axes[k].1.len() {
                    exhausted = false;
                    break;
                }
                idx[k] = 0;
            }
            if exhausted {
                break;
            }
        }
    }
    let class_worlds = if inputs.ambiguous() { class_axis } else { Vec::new() };
    (out, class_worlds)
}
