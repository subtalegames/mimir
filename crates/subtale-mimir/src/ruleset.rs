use std::collections::BTreeMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{evaluator::Evaluator, query::Query, rule::Rule};

/// At a high level, a `Ruleset` is a collection of `Rule` instances that define
/// predicate behaviour for evaluating "facts" in a game world.
pub trait RulesetTrait<FactKey, FactType, FactEvaluator: Evaluator<FactType>, Outcome>
where
    FactKey: std::hash::Hash + Eq,
{
    /// Creates a new ruleset from the provided collection of rules.
    fn new(rules: Vec<Rule<FactKey, FactType, FactEvaluator, Outcome>>) -> Self;

    /// Evaluates the ruleset against the provided query.
    ///
    /// Depending on the implementation, this function may return a single rule
    /// (i.e. the most specific) or multiple rules that evaluate to true for the
    /// provided query.
    fn evaluate(
        &self,
        query: &Query<FactKey, FactType>,
    ) -> Vec<&Rule<FactKey, FactType, FactEvaluator, Outcome>>;
}

/// An implementation of a `Ruleset` that returns all rules that evaluate to
/// true for the provided query, regardless of their specificity. This means
/// that all rules in the ruleset are evaluated, and all rules that evaluate to
/// true are returned.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimpleRuleset<FactKey, FactType, FactEvaluator: Evaluator<FactType>, Outcome>
where
    FactKey: std::hash::Hash + Eq,
{
    /// The collection of rules that make up the ruleset.
    rules: Vec<Rule<FactKey, FactType, FactEvaluator, Outcome>>,
}

impl<
        FactKey: std::hash::Hash + Eq,
        FactType: Copy,
        FactEvaluator: Evaluator<FactType> + Copy,
        Outcome,
    > RulesetTrait<FactKey, FactType, FactEvaluator, Outcome>
    for SimpleRuleset<FactKey, FactType, FactEvaluator, Outcome>
{
    /// Creates a new ruleset from the provided collection of rules.
    fn new(rules: Vec<Rule<FactKey, FactType, FactEvaluator, Outcome>>) -> Self {
        Self { rules }
    }

    /// Evaluates the ruleset against the provided query.
    ///
    /// Returns all rules in the ruleset that evaluate to true for the provided
    /// query.
    fn evaluate(
        &self,
        query: &Query<FactKey, FactType>,
    ) -> Vec<&Rule<FactKey, FactType, FactEvaluator, Outcome>> {
        self.rules
            .iter()
            .filter(|rule| rule.evaluate(query))
            .collect()
    }
}

/// An implementation of a `Ruleset` that returns the most specific rule that
/// evaluates to true for the provided query.
///
/// By default, specificity (weight) is determined by the number of evaluators
/// in the rule. However, this can be overridden by setting the weight of the
/// rule explicitly.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeightedRuleset<FactKey, FactType, FactEvaluator: Evaluator<FactType>, Outcome>
where
    FactKey: std::hash::Hash + Eq,
{
    /// The collection of rules that make up the ruleset, indexed by their weight
    /// (which defaults to the number of evaluators in the rule, but can be
    /// overridden).
    rules: BTreeMap<isize, Vec<Rule<FactKey, FactType, FactEvaluator, Outcome>>>,

    /// The number of rules in the weight with the most rules. This is effectively
    /// the maximum number of rules that will be evaluated for a given query.
    ///
    /// With this, we can pre-allocate the space for the rules that will be evaluated
    /// for a given query.
    largest_weight_cardinality: usize,
}

impl<
        FactKey: std::hash::Hash + Eq,
        FactType: Copy,
        FactEvaluator: Evaluator<FactType> + Copy,
        Outcome,
    > RulesetTrait<FactKey, FactType, FactEvaluator, Outcome>
    for WeightedRuleset<FactKey, FactType, FactEvaluator, Outcome>
{
    /// Creates a new ruleset from the provided collection of rules.
    fn new(rules: Vec<Rule<FactKey, FactType, FactEvaluator, Outcome>>) -> Self {
        let mut ruleset = Self {
            rules: BTreeMap::new(),
            largest_weight_cardinality: 0,
        };

        for rule in rules {
            let weight = rule.evaluators.len() as isize;
            let rules = ruleset.rules.entry(weight).or_default();
            rules.push(rule);

            if rules.len() > ruleset.largest_weight_cardinality {
                ruleset.largest_weight_cardinality = rules.len();
            }
        }

        ruleset
    }

    /// Evaluates the ruleset against the provided query.
    ///
    /// Returns the most specific (most evaluators) rule in the ruleset that
    /// evaluates to true for the provided query. If multiple rules evaluate
    /// to true with the same weight/specificity, they are all returned.
    fn evaluate(
        &self,
        query: &Query<FactKey, FactType>,
    ) -> Vec<&Rule<FactKey, FactType, FactEvaluator, Outcome>> {
        let mut rules = Vec::with_capacity(self.largest_weight_cardinality);

        for (_, rule) in self.rules.iter().rev() {
            for r in rule {
                if r.evaluate(query) {
                    rules.push(r);
                }
            }

            if !rules.is_empty() {
                break;
            }
        }

        rules.shrink_to_fit();

        rules
    }
}

#[cfg(test)]
#[cfg(feature = "float")]
mod tests {
    use crate::prelude::*;

    #[test]
    fn simple_ruleset_init() {
        let mut rule = Rule::new("You killed 5 enemies!");
        rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));

        let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!");
        more_specific_rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));
        more_specific_rule.insert("doors_opened", FloatEvaluator::gt(2.));

        let ruleset = SimpleRuleset::new(vec![rule, more_specific_rule]);

        assert_eq!(ruleset.rules.len(), 2);
    }

    #[test]
    fn simple_ruleset_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies!");
        rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));

        let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!");
        more_specific_rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));
        more_specific_rule.insert("doors_opened", FloatEvaluator::gt(2.));

        let ruleset = SimpleRuleset::new(vec![rule, more_specific_rule]);

        let mut query = Query::new();
        query.insert("enemies_killed", 5.);

        assert_eq!(
            ruleset.evaluate(&query).first().unwrap().outcome,
            "You killed 5 enemies!"
        );

        let mut more_specific_query = Query::new();
        more_specific_query.insert("enemies_killed", 5.);
        more_specific_query.insert("doors_opened", 10.);

        assert_eq!(
            ruleset.evaluate(&more_specific_query).len(), 2,
        );
    }

    #[test]
    fn weighted_ruleset_init() {
        let mut rule = Rule::new("You killed 5 enemies!");
        rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));

        let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!");
        more_specific_rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));
        more_specific_rule.insert("doors_opened", FloatEvaluator::gt(2.));

        let mut another_specific_rule = Rule::new("You killed 5 enemies and collected 10 coins!");
        another_specific_rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));
        another_specific_rule.insert("coins_collected", FloatEvaluator::EqualTo(10.));

        let ruleset = WeightedRuleset::new(vec![rule, more_specific_rule, another_specific_rule]);

        assert_eq!(ruleset.rules.len(), 2);
        assert_eq!(ruleset.largest_weight_cardinality, 2);
    }

    #[test]
    fn weighted_ruleset_evaluation() {
        let mut rule = Rule::new("You killed 5 enemies!");
        rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));

        let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!");
        more_specific_rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));
        more_specific_rule.insert("doors_opened", FloatEvaluator::gt(2.));

        let ruleset = WeightedRuleset::new(vec![rule, more_specific_rule]);

        let mut query = Query::new();
        query.insert("enemies_killed", 2.5 + 1.5 + 1.);

        assert_eq!(
            ruleset.evaluate(&query).first().unwrap().outcome,
            "You killed 5 enemies!"
        );

        let mut more_specific_query = Query::new();
        more_specific_query.insert("enemies_killed", 2.5 + 1.5 + 1.);
        more_specific_query.insert("doors_opened", 10.);

        assert_eq!(
            ruleset.evaluate(&more_specific_query).first().unwrap().outcome,
            "You killed 5 enemies and opened 2 doors!"
        );
    }
}
