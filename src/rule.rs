use std::collections::BTreeMap;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::criterion::Criterion;

#[derive(Default, Serialize, Deserialize)]
pub struct Query(pub BTreeMap<String, f64>);

impl Query {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, symbol: String, value: f64) {
        self.0.insert(symbol, value);
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Rule(pub BTreeMap<String, Criterion>, String);

impl Rule {
    pub fn new(outcome: String) -> Self {
        Self(BTreeMap::new(), outcome)
    }

    pub fn insert(&mut self, symbol: String, criterion: Criterion) {
        self.0.insert(symbol, criterion);
    }
}

impl Rule {
    pub fn evaluate(&self, query: &Query) -> bool {
        for (rule, criterion) in &self.0 {
            if let Some(fact) = query.0.get(rule) {
                if !criterion.evaluate(*fact) {
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
pub struct Ruleset(pub Vec<Rule>);

impl Ruleset {
    pub fn from(mut vec: Vec<Rule>) -> Self {
        vec.sort_by_cached_key(|x| x.0.len());
        vec.reverse();

        Self(vec)
    }

    pub fn evaluate_all(&self, query: &Query) -> Vec<&Rule> {
        let mut matched = Vec::<&Rule>::new();

        for rule in self.0.iter() {
            if matched.get(0).map_or(0, |x| x.0.len()) <= rule.0.len() {
                if rule.evaluate(query) {
                    matched.push(rule);
                }
            } else {
                break;
            }
        }

        matched
    }

    pub fn evaluate(&self, query: &Query) -> Option<&Rule> {
        let matched = self.evaluate_all(query);
        matched.choose(&mut rand::thread_rng()).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies!".to_owned());
        rule.insert("enemies_killed".into(), Criterion::EqualTo(5.));

        let mut query = Query::new();
        query.insert("enemies_killed".into(), 2.5 + 1.5 + 1.);

        assert!(rule.evaluate(&query));
    }

    #[test]
    fn complex_rule_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies and opened 2 doors!".to_owned());
        rule.insert("enemies_killed".into(), Criterion::EqualTo(5.));
        rule.insert("doors_opened".into(), Criterion::gt(2.));

        let mut query = Query::new();
        query.insert("enemies_killed".into(), 2.5 + 1.5 + 1.);
        query.insert("doors_opened".into(), 10.);

        assert!(rule.evaluate(&query));
    }

    #[test]
    fn rule_set_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies!".to_owned());
        rule.insert("enemies_killed".into(), Criterion::EqualTo(5.));

        let mut more_specific_rule =
            Rule::new("You killed 5 enemies and opened 2 doors!".to_owned());
        more_specific_rule.insert("enemies_killed".into(), Criterion::EqualTo(5.));
        more_specific_rule.insert("doors_opened".into(), Criterion::gt(2.));

        let rule_set = Ruleset::from(vec![rule, more_specific_rule]);

        let mut query = Query::new();
        query.insert("enemies_killed".into(), 2.5 + 1.5 + 1.);

        assert_eq!(
            rule_set.evaluate_all(&query)[0].1.as_str(),
            "You killed 5 enemies!"
        );

        let mut more_specific_query = Query::new();
        more_specific_query.insert("enemies_killed".into(), 2.5 + 1.5 + 1.);
        more_specific_query.insert("doors_opened".into(), 10.);

        assert_eq!(
            rule_set.evaluate_all(&more_specific_query)[0].1.as_str(),
            "You killed 5 enemies and opened 2 doors!"
        );
    }
}
