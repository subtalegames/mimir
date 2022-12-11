use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

use crate::criterion::Criterion;

#[derive(Default, Serialize, Deserialize)]
pub struct Query(BTreeMap<String, f64>);

impl Query {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, symbol: String, value: f64) {
        self.0.insert(symbol, value);
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Rule(BTreeMap<String, Criterion>, String);

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
pub struct Ruleset(Vec<Rule>);

impl Ruleset {
    pub fn from(mut vec: Vec<Rule>) -> Self {
        vec.sort_by_cached_key(|x| x.0.len());
        vec.reverse();

        Self(vec)
    }

    pub fn evaluate(&self, query: &Query) -> Option<&Rule> {
        self.0.iter().find(|&rule| rule.evaluate(query))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies!".to_owned());
        rule.insert("enemies_killed".into(), Criterion::eq(5.));

        let mut query = Query::new();
        query.insert("enemies_killed".into(), 2.5 + 1.5 + 1.);

        assert!(rule.evaluate(&query));
    }

    #[test]
    fn complex_rule_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies and opened 2 doors!".to_owned());
        rule.insert("enemies_killed".into(), Criterion::eq(5.));
        rule.insert("doors_opened".into(), Criterion::gt(2.));

        let mut query = Query::new();
        query.insert("enemies_killed".into(), 2.5 + 1.5 + 1.);
        query.insert("doors_opened".into(), 10.);

        assert!(rule.evaluate(&query));
    }

    #[test]
    fn rule_set_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies!".to_owned());
        rule.insert("enemies_killed".into(), Criterion::eq(5.));

        let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!".to_owned());
        more_specific_rule.insert("enemies_killed".into(), Criterion::eq(5.));
        more_specific_rule.insert("doors_opened".into(), Criterion::gt(2.));

        let rule_set = Ruleset::from(vec![rule, more_specific_rule]);

        let mut query = Query::new();
        query.insert("enemies_killed".into(), 2.5 + 1.5 + 1.);

        assert_eq!(rule_set.evaluate(&query).unwrap().1.as_str(), "You killed 5 enemies!");

        let mut more_specific_query = Query::new();
        more_specific_query.insert("enemies_killed".into(), 2.5 + 1.5 + 1.);
        more_specific_query.insert("doors_opened".into(), 10.);

        assert_eq!(rule_set.evaluate(&more_specific_query).unwrap().1.as_str(), "You killed 5 enemies and opened 2 doors!");
    }
}
