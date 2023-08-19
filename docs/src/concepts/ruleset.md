# Ruleset

Rulesets are collections of rules, represented in Rust as `Vec<Rule<...>>`.

```rs
struct Ruleset<FactKey, FactType, FactEvaluator: Evaluator<FactType>, Outcome>
where
    FactKey: std::hash::Hash + std::cmp::Eq,
{
    rules: Vec<Rule<FactKey, FactType, FactEvaluator, Outcome>>,
}
```

> ℹ️ Check out the [ruleset storage section](/performance.html#ruleset-storage) on the performance page for further details on how Mímir represents rulesets in Rust to improve performance when evaluating queries against them.

## Evaluation

Just like rules, rulesets can be evaluated against queries to determine if they are true given the current game world's state:

```rs
let mut rule = Rule::new("You killed 5 enemies!");
rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));

let mut more_specific_rule = Rule::new(
    "You killed 5 enemies and opened 2 doors!"
);
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
```

In the above example, we define a ruleset with two rules. Both rules require that 5 enemies have been killed, but one rule is more specific (also requiring that more than 2 doors have been opened).

The first query evaluates to the simpler rule, because the query does not satisfy the doors opened requirement. However, the second query evaluates to the more complex rule because the query *does* satistfy the doors opened requirement.

> ℹ️ In the second query, although the simpler rule is satisfied, Mímir does not evaluate it as true because it's less specific (i.e. contains fewer evaluators).
