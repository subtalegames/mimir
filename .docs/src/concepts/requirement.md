# Requirement

A `Requirement` is simply a definition of a predicate on a double precision floating-point number (represented in Mímir using Rust's `f64` type).

```rs
enum Requirement {
    EqualTo(f64),
    NotEqualTo(f64),
    LessThan(RangeBound),
    GreaterThan(RangeBound),
    InRange(RangeBound, RangeBound),
}
```

::: info
`RangeBound` is an enum that holds a boundary value that can be inclusive (`RangeBound::Inclusive(f64)`) or exclusive (`RangeBound::Exclusive(f64)`).
:::

## Helper functions

Several helper functions are exposed to easily instantiate requirements with common equality expressions:

| Function             | Internal     | Equivalent to |
| :------------------: | :----------: | :-----------: |
| `Requirement::lt(5.)`  | `Requirement::LessThan(RangeBound::Exclusive(5.))` | `x < 5`       |
| `Requirement::lte(5.)` | `Requirement::LessThan(RangeBound::Inclusive(5.))` | `x ≤ 5`       |
| `Requirement::gt(5.)`  | `Requirement::GreaterThan(RangeBound::Exclusive(5.))`  | `x > 5`       |
| `Requirement::gte(5.)` | `Requirement::GreaterThan(RangeBound::Inclusive(5.))`  | `x ≥ 5`       |
| `Requirement::range(5., 10.)` | `Requirement::InRange(RangeBound::Inclusive(5.), RangeBound::Exclusive(10.))` | `5 ≤ x < 10` |

::: info
`Requirement::range` is designed to mimic the functionality of [Python's built-in range function][py-range].
:::

## Real world

In the real world, a requirement represents a condition that must be true for a contextual event to take place. However, events will typically have many requirements that need to evaluate to true, not just one!

::: info EXAMPLE
An NPC might query Mímir to ensure that they're only commenting on another NPC's behaviour if they've not exhibited the same behaviour previously (to avoid being hypocritical).

*It could also function inversely if the NPC is intentionally a hypocrite!*
:::

## Floating-point equality

Internally, Mímir uses the [float-cmp][float-cmp] crate to perform approximate comparisons between requirements and fact values when `Requirement::EqualTo` or `Requirement::NotEqualTo` are evaluated.


[py-range]: https://docs.python.org/3/library/functions.html#func-range
[float-cmp]: https://crates.io/crates/float-cmp
