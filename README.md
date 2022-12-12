# Mímir

[![Rust](https://github.com/subtalegames/mimir/actions/workflows/rust.yml/badge.svg)](https://github.com/subtalegames/mimir/actions/workflows/rust.yml)

> Mímir is a contextual query engine (implemented in Rust) for video games with dynamic events (e.g. dialog, animations) driven by the current world's state.

## Inspiration

Mímir (both in concept and general architecture) is heavily inspired by [Elan Ruskin's amazing session from GDC 2012 on AI-driven Dynamic Dialog][gdc].

Fundamentally speaking, Mímir is simply a Rust implementation of Elan's proposed system for dynamic dialog. However, Mímir does offer some differences and/or extensions that cater specifically to games developed internally at Subtale (documented below).

## High-level overview

Your game's world is defined as a collection of facts: the player killed x amount of enemies, an NPC has opened y amount of doors, the player is currently near z NPC, etc.

In Mímir, facts are collected together into a map ([`Query`](#query)), where the key is the unique identifier of the fact, and the value is the fact's value (represented as a `f64`).

Also, your game will (most likey!) have predefined rules that define behaviour that should occur when one or more facts are true. We represent rules as a map ([`Rule<T>`](#rule)), where the key is the unique identifier of the fact, and the value is a predicate ([`Criterion`](#criterion)) that acts on the fact's value.

Finally, rules can be stored together in collections known as rulesets ([`Ruleset<T>`](#ruleset)). Rulesets allow a query to be evaluated against many rules at once: Mímir will always look to match a query against the rule in the ruleset with the most criteria (i.e. more specific). *(If multiple rules are matched with the same specificity, one is chosen at random.)*

## Concepts

### Criterion

A `Criterion` is simply a definition of a predicate on a double precision floating-point number (represented in Mímir using Rust's `f64` type).

```rs
enum Criterion {
    EqualTo(f64),
    NotEqualTo(f64),
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
| `Criterion::range(5., 10.)`* | `Criterion::InRange(CriterionBound::Inclusive(5.), CriterionBound::Exclusive(10.))` | `5 ≤ x < 10` |

*: `Criterion::range` is designed to mimic the functionality of [Python's built-in range function][py-range].

#### Real-world

In the real-world, a criterion represents a condition that must be true for a contextual event to take place. However, events will typically have many criteria that need to evaluate to true, not just one!

> For example, an NPC might query Mímir to ensure that they're only commenting on another NPC's behaviour if they've not exhibited the same behaviour previously (to avoid being hypocritical).
>
> *It could also function inversely if the NPC is intentionally a hypocrite!*

#### Floating-point equality comparison

Internally, Mímir uses the [float-cmp](https://crates.io/crates/float-cmp) crate to perform approximate comparisons between criterion and fact values when `Criterion::EqualTo` (and `Criterion::NotEqualTo`) is used.

### Query

A query is a collection of facts about the current game world's state. Mímir represents these facts in Rust as a `BTreeMap<String, f64>`, where the `String` is the unique name of the fact, and the `f64` is the fact's value.

```rs
struct Query {
    facts: BTreeMap<String, f64>,
}
```

### Rule

A `Rule` is a collection of criteria stored in a map (using symbols as keys) with a specific outcome (`T`). Every criterion in the rule must evaluate to true for the rule itself to be considered true.

```rs
struct Rule<T> {
    criteria: BTreeMap<String, Criterion>,
    pub outcome: T,
}
```

#### Evaluating against queries

Rules can be evaluated against queries to determine if they are true given the current game world's state:

```rs
let mut rule = Rule::new(true);
rule.require("enemies_killed".into(), Criterion::eq(5.));

let mut query = Query::new();
query.fact("enemies_killed".into(), 2.5 + 1.5 + 1.);

assert!(rule.evaluate(&query));
```

In the above example, the rule evaluates to true for the supplied query because it's expecting 5 enemies to be killed (`enemies_killed`), and the query confirms the fact that 5 (`2.5 + 1.5 + 1`) have been killed.

> *Our generic outcome type (`T`) for the example is just a standard boolean value (`true`). In the real-world, you'd probably use a more complex enum to denote different types of outcome (e.g. dialog, animation).*

### Ruleset

Rulesets are simply collections of rules (represented in Mímir as `Vec<Rule<T>>`).

```rs
struct Ruleset<T> {
    rules: Vec<Rule<T>>,
}
```

#### Evaluating against queries

Just like rules, rulesets can be evaluated against queries to determine if they are true given the current game world's state:

```rs
let mut rule = Rule::new("You killed 5 enemies!");
rule.require("enemies_killed".into(), Criterion::EqualTo(5.));

let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!");
more_specific_rule.require("enemies_killed".into(), Criterion::EqualTo(5.));
more_specific_rule.require("doors_opened".into(), Criterion::gt(2.));

let ruleset = Ruleset::from(vec![rule, more_specific_rule]);

let mut query = Query::new();
query.fact("enemies_killed".into(), 2.5 + 1.5 + 1.);

assert_eq!(
    ruleset.evaluate_all(&query)[0].outcome,
    "You killed 5 enemies!"
);

let mut more_specific_query = Query::new();
more_specific_query.fact("enemies_killed".into(), 2.5 + 1.5 + 1.);
more_specific_query.fact("doors_opened".into(), 10.);

assert_eq!(
    ruleset.evaluate_all(&more_specific_query)[0].outcome,
    "You killed 5 enemies and opened 2 doors!"
);
```

In the above example, we define a ruleset with two rules. Both rules require that 5 enemies have been killed, but one rule is more specific (also requiring that more than 2 doors have been opened).

The first query evaluates to the simpler rule, because the query does not satisfy the doors opened requirement. However, the second query evaluates to the more complex rule because the query *does* satistfy the doors opened requirement (note that even though the simpler rule is still satisfied, Mímir does not evaluate it as true because it's less specific/contains less criteria).

## Serialization

Criteria (including bounds), rules, and rulesets are all (de)serializable using [serde](https://serde.rs/) if you enable the respective feature in your project's `Cargo.toml`:

```toml
[dependencies]
mimir = { version = "x.x.x", features = ["serde"] }
```

This makes it easy for you to serialize rulesets into a persistent medium (i.e. files) during your game's development process, bundle them with your game, and deserialize them at runtime.

> *This also means that Mímir can effortlessly support modding by allowing you to deserialize and load user-defined rulesets at runtime.*

## Performance

> 🚧 Please note that while Mímir is in a pre-release state (<1.0.0), this section is a WIP and constantly evolving.

### Ruleset storage

Because Mímir evaluates rulesets by returning the most specific rule for a given query, the rules are stored in descending order of criteria length. This avoids scanning the entire ruleset for matching rules, as the first rules in the underlying collection are the most specific.

However, this does mean that care should be taken when invoking `ruleset.append(...)` to introduce more rules into a ruleset, as this function also triggers the underlying collection to be sorted again after the new rules are appended. *(Ideally, the rulesets should be manipulated during your game's loading state, and then only evaluated during your game's main loop.)*

### Multiple rulesets

Where possible, you should look to divide your game's entire database of rules into smaller rulesets that can be loaded in and out of memory depending on the game's current state.

For example, you might want to partition your rules into individual rulesets for each level/map/region of your game. Otherwise, you'll be subjecting yourself to an unnecessary performance cost by having Mímir evaluate rules that have no relevance to the game's current state.

> *The specific implementation of a system as described above is outside the scope of Mímir.*

## Why Mímir?

Quoting from [norse-mythology.org][mimir], Mímir is an exceptionally wise being and a counselor of the gods. We hope that this contextual query engine lives up to its name in being just as wise about your game's state and defined behaviours!

*P.S. We also love all of the work done by the folks over at [Santa Monica Studio][sms] (especially 2018's reboot of God of War)!*

[gdc]: https://www.youtube.com/watch?v=tAbBID3N64A
[py-range]: https://docs.python.org/3/library/functions.html#func-range
[mimir]: https://norse-mythology.org/gods-and-creatures/others/mimir/
[sms]: https://sms.playstation.com
