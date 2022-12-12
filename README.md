# Mimir

> 🧠 Contextual query engine for dynamic video games

Mimir is a contextual query engine (implemented in Rust) for video games with dynamic "events" (e.g. dialog, animations) driven by the current world's "state" (context).

## Inspiration

Mimir (both in concept and general architecture) is heavily inspired by [Elan Ruskin's amazing session from GDC 2012 on AI-driven Dynamic Dialog][gdc].

At a high-level, Mimir is simply a Rust implementation of Elan's proposed system for dynamic dialog. However, Mimir does offer some differences and/or extensions that cater specifically to games developed internally at Subtale (documented below).

## Concepts

### Criterion

A `Criterion` is simply a definition of a predicate on a double precision floating-point number (represented in Mimir using Rust's `f64` type).

```rs
enum Criterion {
  EqualTo(f64),
  LessThan(CriterionBound),
  GreaterThan(CriterionBound),
  InRange(CriterionBound, CriterionBound),
}
```

> *`CriterionBound` is an enum that holds a boundary value that can be inclusive (`CriterionBound::Inclusive(f64)`) or exclusive (`CriterionBound::Exclusive(f64)`).*

#### Helper functions

Several helper functions are exposed to easily instantiate criteria with common equality expressions:

| Function             | Internal     | Equivalent to |
| :------------------: | :----------: | :-----------: |
| `Criterion::lt(5.)`  | `Criterion::LessThan(CriterionBound::Exclusive(5.))` | `x < 5`       |
| `Criterion::lte(5.)` | `Criterion::LessThan(CriterionBound::Inclusive(5.))` | `x ≤ 5`       |
| `Criterion::gt(5.)`  | `Criterion::GreaterThan(CriterionBound::Exclusive(5.))`  | `x > 5`       |
| `Criterion::gte(5.)` | `Criterion::GreaterThan(CriterionBound::Inclusive(5.))`  | `x ≥ 5`       |

#### Real-world

In the real-world, a criterion represents a condition that must be true for a contextual event to take place. However, events will typically have many criteria that need to evaluate to true, not just one!

> For example, an NPC might query Mimir to ensure that they're only commenting on another NPC's behaviour if they've not exhibited the same behaviour previously (to avoid being hypocritical).
>
> *It could also function inversely if the NPC is intentionally a hypocrite!*

### Query

A query is a collection of "facts" about the current game world's state. Mimir represents these facts in Rust as a `BTreeMap<String, f64>`, where the `String` is the unique name of the fact, and the `f64` is the fact's value.

```rs
struct Query {
  facts: BTreeMap<String, f64>,
};
```

### Rules

A `Rule` is a collection of criteria stored in a map (using symbols as keys) with a specific outcome. Every criterion in the rule must evaluate to true for the rule itself to be considered true.

```rs
struct Rule {
  criteria: BTreeMap<String, Criterion>,
  pub outcome: Outcome,
};
```

#### Evaluating against queries

Rules can be evaluated against queries to determine if they are true given the current game world's state:

```rs
let mut rule = Rule::new(Outcome::Debug("You killed 5 enemies!".into()));
rule.require("enemies_killed".into(), Criterion::eq(5.));

let mut query = Query::new();
query.fact("enemies_killed".into(), 2.5 + 1.5 + 1.);

assert!(rule.evaluate(&query));
```

In the above example, the rule evaluates to true for the supplied query because it's expecting 5 enemies to be killed (`enemies_killed`), and the query confirms the fact that 5 (`2.5 + 1.5 + 1`) have been killed.

### Rulesets

Rulesets are simply collections of rules (represented in Mimir as `Vec<Rule>`).

```rs
struct Ruleset {
  rules: Vec<Rule>,
};
```

[gdc]: https://www.youtube.com/watch?v=tAbBID3N64A