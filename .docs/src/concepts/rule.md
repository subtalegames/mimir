# Rule

A `Rule` is a collection of facts and their requirements stored in a map, along with a specific outcome (`Outcome`). All requirements in the rule must evaluate to true for the rule itself to be considered true.

```rs
struct Rule<FactKey, Outcome> {
    requirements: HashMap<FactKey, Requirement>,
    pub outcome: Outcome,
}
```

## Evaluating against queries

Rules can be evaluated against queries to determine if they are true given the current game world's state:

```rs
let mut rule = Rule::new(true);
rule.require("enemies_killed", Requirement::eq(5.));

let mut query = Query::new();
query.fact("enemies_killed", 2.5 + 1.5 + 1.);

assert!(rule.evaluate(&query));
```

In the above example, the rule evaluates to true for the supplied query because it's expecting 5 enemies to be killed (`enemies_killed`), and the query confirms the fact that 5 (`2.5 + 1.5 + 1`) have been killed.

::: info
Our generic outcome type (`Outcome`) for the example is just a standard boolean value (`true`). In the real-world, you'd probably use a more complex enum to denote different types of outcome (e.g. dialog, animation).
:::