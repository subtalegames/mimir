# Introduction

Mímir is a contextual query engine for video games with dynamic events (e.g. dialog, animations) driven by their current world's state.

```rs
    use subtale_mimir::prelude::*;

    // create a rule requiring that five enemies have been killed
    let mut rule = Rule::new("You killed 5 enemies!");
    // Rule<&str, f64, FloatEvaluator, &str>
    rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));

    // create a more specific rule that also requires at least 2 doors to have been opened
    let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!");
    more_specific_rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));
    more_specific_rule.insert("doors_opened", FloatEvaluator::gte(2.));

    // bundle the rules into a ruleset
    let ruleset = Ruleset::new(vec![rule, more_specific_rule]);

    // run a query against the ruleset
    let mut query = Query::new();
    // Query<&str, f64>
    query.insert("enemies_killed", 2.5 + 1.5 + 1.);

    assert_eq!(
        ruleset.evaluate(&query).unwrap().outcome,
        "You killed 5 enemies!"
    );

    // run a more specific query against the ruleset
    let mut more_specific_query = Query::new();
    more_specific_query.insert("enemies_killed", 2.5 + 1.5 + 1.);
    more_specific_query.insert("doors_opened", 10.);

    assert_eq!(
        ruleset.evaluate(&more_specific_query).unwrap().outcome,
        "You killed 5 enemies and opened 2 doors!"
    );
    ```
