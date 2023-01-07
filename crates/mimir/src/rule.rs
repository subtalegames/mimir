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

    pub fn fact(&mut self, fact: FactKey, value: FactType) {
        self.facts.insert(fact, value);
    }

    pub fn append(&mut self, query: Query<FactKey, FactType>) {
        self.facts.extend(query.facts);
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Rule<FactKey, FactType, Requirement: Evaluator<FactType>, Outcome> {
    marker: PhantomData<FactType>,
    requirements: HashMap<FactKey, Requirement>,
    pub outcome: Outcome,
}

impl<
        FactKey: std::hash::Hash + std::cmp::Eq,
        FactType: std::marker::Copy,
        Requirement: Evaluator<FactType> + std::marker::Copy,
        Outcome,
    > Rule<FactKey, FactType, Requirement, Outcome>
{
    pub fn new(outcome: Outcome) -> Self {
        Self {
            marker: PhantomData,
            requirements: HashMap::new(),
            outcome,
        }
    }

    pub fn require(&mut self, fact: FactKey, requirement: Requirement) {
        self.requirements.insert(fact, requirement);
    }

    pub fn evaluate(&self, query: &Query<FactKey, FactType>) -> bool {
        for (fact, requirement) in &self.requirements {
            if let Some(fact_value) = query.facts.get(fact) {
                if !requirement.evaluate(*fact_value) {
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
pub struct Ruleset<FactKey, FactType, Requirement: Evaluator<FactType>, Outcome> {
    rules: Vec<Rule<FactKey, FactType, Requirement, Outcome>>,
}

impl<
        FactKey: std::hash::Hash + std::cmp::Eq,
        FactType: std::marker::Copy,
        Requirement: Evaluator<FactType> + std::marker::Copy,
        Outcome,
    > Ruleset<FactKey, FactType, Requirement, Outcome>
{
    fn sort(&mut self) {
        self.rules.sort_by_cached_key(|x| x.requirements.len());
        self.rules.reverse();
    }

    pub fn new(rules: Vec<Rule<FactKey, FactType, Requirement, Outcome>>) -> Self {
        let mut new = Self { rules };
        new.sort();
        new
    }

    pub fn append(&mut self, ruleset: &mut Ruleset<FactKey, FactType, Requirement, Outcome>) {
        self.rules.append(&mut ruleset.rules);
        self.sort();
    }

    pub fn evaluate_all(
        &self,
        query: &Query<FactKey, FactType>,
    ) -> Vec<&Rule<FactKey, FactType, Requirement, Outcome>> {
        let mut matched = Vec::<&Rule<FactKey, FactType, Requirement, Outcome>>::new();

        for rule in self.rules.iter() {
            if matched.get(0).map_or(0, |x| x.requirements.len()) <= rule.requirements.len() {
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
    ) -> Option<&Rule<FactKey, FactType, Requirement, Outcome>> {
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
        rule.require("enemies_killed", FloatEvaluator::EqualTo(5.));

        let mut query = Query::new();
        query.fact("enemies_killed", 2.5 + 1.5 + 1.);

        assert!(rule.evaluate(&query));
    }

    #[test]
    fn complex_rule_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies and opened 2 doors!");
        rule.require("enemies_killed", FloatEvaluator::EqualTo(5.));
        rule.require("doors_opened", FloatEvaluator::gt(2.));

        let mut query = Query::new();
        query.fact("enemies_killed", 2.5 + 1.5 + 1.);
        query.fact("doors_opened", 10.);

        assert!(rule.evaluate(&query));
    }

    #[test]
    fn ruleset_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies!");
        rule.require("enemies_killed", FloatEvaluator::EqualTo(5.));

        let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!");
        more_specific_rule.require("enemies_killed", FloatEvaluator::EqualTo(5.));
        more_specific_rule.require("doors_opened", FloatEvaluator::gt(2.));

        let ruleset = Ruleset::new(vec![rule, more_specific_rule]);

        let mut query = Query::new();
        query.fact("enemies_killed", 2.5 + 1.5 + 1.);

        assert_eq!(
            ruleset.evaluate_all(&query)[0].outcome,
            "You killed 5 enemies!"
        );

        let mut more_specific_query = Query::new();
        more_specific_query.fact("enemies_killed", 2.5 + 1.5 + 1.);
        more_specific_query.fact("doors_opened", 10.);

        assert_eq!(
            ruleset.evaluate_all(&more_specific_query)[0].outcome,
            "You killed 5 enemies and opened 2 doors!"
        );
    }
}
