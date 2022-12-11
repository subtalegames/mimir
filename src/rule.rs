use std::collections::BTreeMap;

use symbol::Symbol;

use crate::criterion::Criterion;

#[derive(Default)]
pub struct Query(BTreeMap<Symbol, f64>);

impl Query {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, symbol: Symbol, value: f64) {
        self.0.insert(symbol, value);
    }
}

#[derive(Default)]
pub struct Rule(BTreeMap<Symbol, Criterion>);

impl Rule {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, symbol: Symbol, criterion: Criterion) {
        self.0.insert(symbol, criterion);
    }
}

impl Rule {
    pub fn evaluate(&self, query: &Query) -> bool {
        for (query_key, query_value) in &query.0 {
            if let Some(rule_criterion) = self.0.get(query_key) {
                if !rule_criterion.evaluate(*query_value) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_evaluation() {
        let mut rule = Rule::new();
        rule.insert("enemies_killed".into(), Criterion::eq(5.));

        let mut query = Query::new();
        query.insert("enemies_killed".into(), 2.5 + 1.5 + 1.);

        assert!(rule.evaluate(&query));
    }
}
