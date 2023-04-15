use std::marker::PhantomData;

use indexmap::IndexMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{evaluator::Evaluator, query::Query};

/// A `Rule` is a collection of facts and their evaluators (requirements) stored in a map, along with a specific
/// outcome (`Outcome`). All evaluators in a rule must evaluate to `true` for the rule itself to be considered
/// `true`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rule<FactKey, FactType, FactEvaluator: Evaluator<FactType>, Outcome>
where
    FactKey: std::hash::Hash + std::cmp::Eq,
{
    marker: PhantomData<FactType>,
    /// The map of facts and evaluators that will be used to evaluate each fact's value.
    pub evaluators: IndexMap<FactKey, FactEvaluator>,
    /// The outcome of the rule that's returned during evaluation if the rule matches the supplied `Query` instance.
    pub outcome: Outcome,
}

impl<
        FactKey: std::hash::Hash + std::cmp::Eq,
        FactType: std::marker::Copy,
        FactEvaluator: Evaluator<FactType> + std::marker::Copy,
        Outcome,
    > Rule<FactKey, FactType, FactEvaluator, Outcome>
{
    /// Instantiates a new instance of `Rule` without allocating an underlying collection of evaluators.
    ///
    /// Computes in `O(1)` time.
    pub fn new(outcome: Outcome) -> Self {
        Self {
            marker: PhantomData,
            evaluators: IndexMap::new(),
            outcome,
        }
    }

    /// Inserts a new evaluator for a specific fact key into the rule.
    ///
    /// Computes in `O(1)` time (amortized average, depending on current capacity).
    pub fn insert(&mut self, fact: FactKey, evaluator: FactEvaluator) {
        self.evaluators.insert(fact, evaluator);
    }

    /// Evaluates the rule against the provided query.
    ///
    /// Returns `true` if all facts in the rule are present in the query and all fact evaluators
    /// resolve to `true`, otherwise returns `false`.
    ///
    /// Computes in `O(n)` time (worst case). This is dependent on your evaluator implementation
    /// evaluating in a constant time.
    pub fn evaluate(&self, query: &Query<FactKey, FactType>) -> bool {
        // IndexMap::len() has a time complexity of O(1), so we check this
        // against the query's length to avoid unnecessary iteration
        if self.evaluators.len() > query.facts.len() {
            return false;
        }

        // Iterate over all evaluators. If any evaluator is not found
        // in the query or evaluates to false, break out of the loop
        // and return false
        for (fact, evaluator) in &self.evaluators {
            if let Some(fact_value) = query.facts.get(fact) {
                if !evaluator.evaluate(*fact_value) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // All evaluators were found in the query, and all evaluated
        // to true, so the rule is true for the provided query
        true
    }
}

#[cfg(test)]
#[cfg(feature = "float")]
mod tests {
    use super::*;

    use crate::float::FloatEvaluator;

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
}
