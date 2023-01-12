use std::{collections::HashMap, marker::PhantomData};

use rand::seq::SliceRandom;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::evaluator::Evaluator;

#[derive(Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Query<FactKey, FactType> {
    facts: HashMap<FactKey, FactType>,
}

impl<FactKey: std::hash::Hash + std::cmp::Eq, FactType: std::marker::Copy>
    Query<FactKey, FactType>
{
    pub fn new() -> Self {
        Self {
            facts: HashMap::new(),
        }
    }

    pub fn insert(&mut self, fact: FactKey, value: FactType) {
        self.facts.insert(fact, value);
    }

    pub fn extend(&mut self, query: Query<FactKey, FactType>) {
        self.facts.extend(query.facts);
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Rule<FactKey, FactType, FactEvaluator: Evaluator<FactType>, Outcome> {
    marker: PhantomData<FactType>,
    evaluators: HashMap<FactKey, FactEvaluator>,
    pub outcome: Outcome,
}

impl<
        FactKey: std::hash::Hash + std::cmp::Eq,
        FactType: std::marker::Copy,
        FactEvaluator: Evaluator<FactType> + std::marker::Copy,
        Outcome,
    > Rule<FactKey, FactType, FactEvaluator, Outcome>
{
    pub fn new(outcome: Outcome) -> Self {
        Self {
            marker: PhantomData,
            evaluators: HashMap::new(),
            outcome,
        }
    }

    pub fn insert(&mut self, fact: FactKey, evaluator: FactEvaluator) {
        self.evaluators.insert(fact, evaluator);
    }

    pub fn evaluate(&self, query: &Query<FactKey, FactType>) -> bool {
        for (fact, evaluator) in &self.evaluators {
            if let Some(fact_value) = query.facts.get(fact) {
                if !evaluator.evaluate(*fact_value) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Ruleset<FactKey, FactType, FactEvaluator: Evaluator<FactType>, Outcome> {
    rules: Vec<Rule<FactKey, FactType, FactEvaluator, Outcome>>,
}

impl<
        FactKey: std::hash::Hash + std::cmp::Eq,
        FactType: std::marker::Copy,
        FactEvaluator: Evaluator<FactType> + std::marker::Copy,
        Outcome,
    > Ruleset<FactKey, FactType, FactEvaluator, Outcome>
{
    fn sort(&mut self) {
        self.rules.sort_by_cached_key(|x| x.evaluators.len());
        self.rules.reverse();
    }

    pub fn new(rules: Vec<Rule<FactKey, FactType, FactEvaluator, Outcome>>) -> Self {
        let mut new = Self { rules };
        new.sort();
        new
    }

    pub fn append(&mut self, ruleset: &mut Ruleset<FactKey, FactType, FactEvaluator, Outcome>) {
        self.rules.append(&mut ruleset.rules);
        self.sort();
    }

    pub fn evaluate_all(
        &self,
        query: &Query<FactKey, FactType>,
        ) -> Vec<&Rule<FactKey, FactType, FactEvaluator, Outcome>> {
        let mut matched = Vec::<&Rule<FactKey, FactType, FactEvaluator, Outcome>>::new();

        for rule in self.rules.iter() {
            if matched.get(0).map_or(0, |x| x.evaluators.len()) <= rule.evaluators.len() {
                if rule.evaluate(query) {
                    matched.push(rule);
                }
            } else {
                break;
            }
        }

        matched
    }

    pub fn evaluate(
        &self,
        query: &Query<FactKey, FactType>,
        ) -> Option<&Rule<FactKey, FactType, FactEvaluator, Outcome>> {
        let matched = self.evaluate_all(query);
        matched.choose(&mut rand::thread_rng()).copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::FloatEvaluator;

    use super::*;

    #[test]
    fn rule_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies!");
        rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));

        let mut query = Query::new();
        query.insert("enemies_killed", 2.5 + 1.5 + 1.);

        assert!(rule.evaluate(&query));
    }

    #[test]
    fn complex_rule_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies and opened 2 doors!");
        rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));
        rule.insert("doors_opened", FloatEvaluator::gt(2.));

        let mut query = Query::new();
        query.insert("enemies_killed", 2.5 + 1.5 + 1.);
        query.insert("doors_opened", 10.);

        assert!(rule.evaluate(&query));
    }

    #[test]
    fn ruleset_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies!");
        rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));

        let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!");
        more_specific_rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));
        more_specific_rule.insert("doors_opened", FloatEvaluator::gt(2.));

        let ruleset = Ruleset::new(vec![rule, more_specific_rule]);

        let mut query = Query::new();
        query.insert("enemies_killed", 2.5 + 1.5 + 1.);

        assert_eq!(
            ruleset.evaluate(&query).unwrap().outcome,
            "You killed 5 enemies!"
        );

        let mut more_specific_query = Query::new();
        more_specific_query.insert("enemies_killed", 2.5 + 1.5 + 1.);
        more_specific_query.insert("doors_opened", 10.);

        assert_eq!(
            ruleset.evaluate(&more_specific_query).unwrap().outcome,
            "You killed 5 enemies and opened 2 doors!"
        );
    }
}
