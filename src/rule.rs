use std::collections::BTreeMap;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::criterion::Criterion;

#[derive(Default, Serialize, Deserialize)]
pub struct Query {
    facts: BTreeMap<String, f64>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            facts: BTreeMap::new(),
        }
    }

    pub fn fact(&mut self, fact: String, value: f64) {
        self.facts.insert(fact, value);
    }

    pub fn append(&mut self, query: &mut Query) {
        self.facts.append(&mut query.facts);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Rule<T> {
    criteria: BTreeMap<String, Criterion>,
    pub outcome: T,
}

impl<T> Rule<T> {
    pub fn new(outcome: T) -> Self {
        Self {
            criteria: BTreeMap::new(),
            outcome,
        }
    }

    pub fn require(&mut self, fact: String, criterion: Criterion) {
        self.criteria.insert(fact, criterion);
    }

    pub fn evaluate(&self, query: &Query) -> bool {
        for (fact, criterion) in &self.criteria {
            if let Some(fact_value) = query.facts.get(fact) {
                if !criterion.evaluate(*fact_value) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[derive(Serialize, Deserialize)]
pub struct Ruleset<T> {
    rules: Vec<Rule<T>>,
}

impl<T> Ruleset<T> {
    pub fn from(mut rules: Vec<Rule<T>>) -> Self {
        rules.sort_by_cached_key(|x| x.criteria.len());
        rules.reverse();

        Self {
            rules,
        }
    }

    pub fn append(&mut self, ruleset: &mut Ruleset<T>) {
        self.rules.append(&mut ruleset.rules);
        self.rules.sort_by_cached_key(|x| x.criteria.len());
        self.rules.reverse();
    }

    pub fn evaluate_all(&self, query: &Query) -> Vec<&Rule<T>> {
        let mut matched = Vec::<&Rule<T>>::new();

        for rule in self.rules.iter() {
            if matched.get(0).map_or(0, |x| x.criteria.len()) <= rule.criteria.len() {
                if rule.evaluate(query) {
                    matched.push(rule);
                }
            } else {
                break;
            }
        }

        matched
    }

    pub fn evaluate(&self, query: &Query) -> Option<&Rule<T>> {
        let matched = self.evaluate_all(query);
        matched.choose(&mut rand::thread_rng()).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    pub enum Outcome {
        Debug(String),
    }

    #[test]
    fn rule_evaluation() {
        let mut rule = Rule::new(Outcome::Debug("You killed 5 enemies!".into()));
        rule.require("enemies_killed".into(), Criterion::EqualTo(5.));

        let mut query = Query::new();
        query.fact("enemies_killed".into(), 2.5 + 1.5 + 1.);

        assert!(rule.evaluate(&query));
    }

    #[test]
    fn complex_rule_evaluation() {
        let mut rule = Rule::new(Outcome::Debug("You killed 5 enemies and opened 2 doors!".into()));
        rule.require("enemies_killed".into(), Criterion::EqualTo(5.));
        rule.require("doors_opened".into(), Criterion::gt(2.));

        let mut query = Query::new();
        query.fact("enemies_killed".into(), 2.5 + 1.5 + 1.);
        query.fact("doors_opened".into(), 10.);

        assert!(rule.evaluate(&query));
    }

    #[test]
    fn rule_set_evaluation() {
        let mut rule = Rule::new(Outcome::Debug("You killed 5 enemies!".into()));
        rule.require("enemies_killed".into(), Criterion::EqualTo(5.));

        let mut more_specific_rule =
            Rule::new(Outcome::Debug("You killed 5 enemies and opened 2 doors!".into()));
        more_specific_rule.require("enemies_killed".into(), Criterion::EqualTo(5.));
        more_specific_rule.require("doors_opened".into(), Criterion::gt(2.));

        let rule_set = Ruleset::from(vec![rule, more_specific_rule]);

        let mut query = Query::new();
        query.fact("enemies_killed".into(), 2.5 + 1.5 + 1.);

        assert_eq!(
            rule_set.evaluate_all(&query)[0].outcome,
            Outcome::Debug("You killed 5 enemies!".into())
        );

        let mut more_specific_query = Query::new();
        more_specific_query.fact("enemies_killed".into(), 2.5 + 1.5 + 1.);
        more_specific_query.fact("doors_opened".into(), 10.);

        assert_eq!(
            rule_set.evaluate_all(&more_specific_query)[0].outcome,
            Outcome::Debug("You killed 5 enemies and opened 2 doors!".into())
        );
    }
}
