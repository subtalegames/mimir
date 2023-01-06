# Evaluator

An evaluator is a trait that represents a predicate function evaluating against a value.

```rs
trait Evaluator<T> {
    fn evaluate(self, value: T) -> bool;
}
```

Specifically, in the context of Mímir, an evaluator checks if the value of a fact about the game's current state matches a certain condition.

You can choose to create your own implementation of the trait, or use the provided `FloatEvaluator` implementation (see below) that allows you to evaluate floating-point numbers (Rust's `f64` type).

## Real world

In the real world, an evaluator represents a condition that must be true for a contextual event to take place. However, events will typically have many evaluators that need to evaluate to true, not just one!

::: info EXAMPLE
An NPC might query Mímir to ensure that they're only commenting on another NPC's behaviour if they've not exhibited the same behaviour previously (to avoid being hypocritical).

*It could also function inversely if the NPC is intentionally a hypocrite!*
:::

## FloatEvaluator

::: info
To use the pre-made `FloatEvaluator` implementation, you must enable the `float` feature in your project's `Cargo.toml`:

```toml
[dependencies]
mimir = { version = "x.x.x", features = ["float"] }
```
:::

The `FloatEvaluator` is a built-in implementation of the `Evaluator<T>` trait, allowing you to define [requirements](/concepts/requirement) that match against floating-point numbers.

```rs
enum FloatEvaluator {
    EqualTo(f64),
    NotEqualTo(f64),
    LessThan(FloatRangeBound),
    GreaterThan(FloatRangeBound),
    InRange(FloatRangeBound, FloatRangeBound),
}
```

::: info
`FloatRangeBound` is an enum that holds a boundary value that can be inclusive (`FloatRangeBound::Inclusive(f64)`) or exclusive (`FloatRangeBound::Exclusive(f64)`).
:::

### Helper functions

Several helper functions are exposed to easily instantiate `FloatEvaluator` with common equality expressions:

|             Function             |                                               Internal                                               | Equivalent to |
|:--------------------------------:|:----------------------------------------------------------------------------------------------------:|:-------------:|
|     `FloatEvaluator::lt(5.)`     |                      `FloatEvaluator::LessThan(FloatRangeBound::Exclusive(5.))`                      |    `x < 5`    |
|    `FloatEvaluator::lte(5.)`     |                      `FloatEvaluator::LessThan(FloatRangeBound::Inclusive(5.))`                      |    `x ≤ 5`    |
|     `FloatEvaluator::gt(5.)`     |                    `FloatEvaluator::GreaterThan(FloatRangeBound::Exclusive(5.))`                     |    `x > 5`    |
|    `FloatEvaluator::gte(5.)`     |                    `FloatEvaluator::GreaterThan(FloatRangeBound::Inclusive(5.))`                     |    `x ≥ 5`    |
| `FloatEvaluator::range(5., 10.)` | `FloatEvaluator::InRange(FloatRangeBound::Inclusive(5.), RangeFloatRangeBoundBound::Exclusive(10.))` | `5 ≤ x < 10`  |

::: info
`FloatEvaluator::range` is designed to mimic the functionality of [Python's built-in range function][py-range].
:::

### Floating-point equality

Internally, Mímir's `FloatEvaluator` uses the [float-cmp][float-cmp] crate to perform approximate comparisons when `FloatEvaluator::EqualTo` or `FloatEvaluator::NotEqualTo` are evaluated.

[py-range]: https://docs.python.org/3/library/functions.html#func-range
[float-cmp]: https://crates.io/crates/float-cmp