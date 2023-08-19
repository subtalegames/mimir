# Rule

A `Rule` is a collection of facts and their evaluators (requirements) stored in a map, along with a specific outcome (`Outcome`). All evaluators in the rule must evaluate to true for the rule itself to be considered true.

```rs
struct Rule<FactKey, FactType, FactEvaluator: Evaluator<FactType>, Outcome>
where
    FactKey: std::hash::Hash + std::cmp::Eq,
{
    marker: PhantomData<FactType>,
    evaluators: IndexMap<FactKey, FactEvaluator>,
    pub outcome: Outcome,
}
```

## Evaluation

Rules can be evaluated against queries to determine if they are true given the current game world's state:

```rs
let mut rule = Rule::new(true);
rule.insert("enemies_killed", FloatEvaluator::eq(5.));

let mut query = Query::new();
query.insert("enemies_killed", 2.5 + 1.5 + 1.);

assert!(rule.evaluate(&query));
```

In the above example, the rule evaluates to true for the supplied query because it's expecting 5 enemies to be killed (`enemies_killed`), and the query confirms the fact that 5 (`2.5 + 1.5 + 1`) have been killed.

> ℹ️ Our generic outcome type (`Outcome`) for the example is just a standard boolean value (`true`). In the real-world, you'd probably use a more complex enum to denote different types of outcome (e.g. dialog, animation).

## Insertion order

Mímir stored rule facts and evaluators inside an [`IndexMap`][indexmap] which preserves the insertion order of evaluators.

This distinction is important because it allows you to extract more performance when evaluating rules by ensuring that your "namespace" facts are inserted into the rule first.

For example, imagine a scenario where you're using Mímir to handle character dialog. By establishing a fact that identifies who's speaking (e.g. `"speaker"`), and having the evaluator for the speaker at the beginning of each rule, you can improve performance substantially (because the rule will stop iterating over its remaining evaluators if it finds one that evaluates to false).

[indexmap]: https://github.com/bluss/indexmap
