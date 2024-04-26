use rand::seq::SliceRandom;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{evaluator::Evaluator, query::Query, rule::Rule};

/// A `Ruleset` is a collection of `Rule` instances, represented as a
/// `Vec<Rule<...>>`.
///
/// Because Mímir evaluates rulesets by returning the most specific rule for a
/// given query, the rules are stored in descending order of requirement count.
/// This avoids scanning the entire ruleset for matching rules, as the first
/// rules in the underlying collection are the most specific.
///
/// Where possible, you should look to divide your game's entire database of
/// rules into smaller rulesets that can be loaded in and out of memory
/// depending on the game's current state.
///
/// For example, you might want to partition your rules into individual rulesets
/// for each level/map/region of your game. Otherwise, you'll be subjecting
/// yourself to an unnecessary performance cost by having Mímir evaluate rules
/// that have no relevance to the game's current state.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ruleset<FactKey, FactType, FactEvaluator: Evaluator<FactType>, Outcome>
where
    FactKey: std::hash::Hash + Eq,
{
    rules: Vec<Rule<FactKey, FactType, FactEvaluator, Outcome>>,
}

impl<
        FactKey: std::hash::Hash + Eq,
        FactType: Copy,
        FactEvaluator: Evaluator<FactType> + Copy,
        Outcome,
    > Ruleset<FactKey, FactType, FactEvaluator, Outcome>
{
    fn sort(&mut self) {
        self.rules
            .sort_unstable_by_key(|x| -(x.evaluators.len() as isize));
    }

    /// Creates a new ruleset from the provided collection of rules.
    pub fn new(rules: Vec<Rule<FactKey, FactType, FactEvaluator, Outcome>>) -> Self {
        let mut new = Self { rules };
        new.sort();
        new
    }

    /// Appends all rules from another ruleset into the ruleset.
    pub fn append(&mut self, ruleset: &mut Ruleset<FactKey, FactType, FactEvaluator, Outcome>) {
        self.rules.append(&mut ruleset.rules);
        self.sort();
    }

    /// Evaluates the ruleset against the provided query.
    ///
    /// Returns the most specific (most evaluators) rule in the ruleset that
    /// evaluates to true for the provided query. If multiple rules evaluate
    /// to true with the same specificness, they are all returned.
    pub fn evaluate_all(
        &self,
        query: &Query<FactKey, FactType>,
    ) -> Vec<&Rule<FactKey, FactType, FactEvaluator, Outcome>> {
        let mut matched = Vec::<&Rule<FactKey, FactType, FactEvaluator, Outcome>>::new();

        for rule in self.rules.iter() {
            if matched.first().map_or(0, |x| x.evaluators.len()) <= rule.evaluators.len() {
                if rule.evaluate(query) {
                    matched.push(rule);
                }
            } else {
                break;
            }
        }

        matched
    }

    /// Evaluates the ruleset against the provided query.
    ///
    /// Returns the most specific (most evaluators) rule in the ruleset that
    /// evaluates to true for the provided query. If multiple rules evaluate
    /// to true with the same specificness, one is picked at random.
    pub fn evaluate(
        &self,
        query: &Query<FactKey, FactType>,
    ) -> Option<&Rule<FactKey, FactType, FactEvaluator, Outcome>> {
        let matched = self.evaluate_all(query);
        matched.choose(&mut rand::thread_rng()).copied()
    }
}

#[cfg(test)]
#[cfg(feature = "float")]
mod tests {
    use crate::prelude::*;

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
