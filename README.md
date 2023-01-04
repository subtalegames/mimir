# Mímir

[![Rust](https://github.com/subtalegames/mimir/actions/workflows/rust.yml/badge.svg)](https://github.com/subtalegames/mimir/actions/workflows/rust.yml)

> Mímir is a contextual query engine (implemented in Rust) for video games with dynamic events (e.g. dialog, animations) driven by the current world's state.

## Documentation

Our official documentation at **[mimir.subtale.com][docs]** offers a guide with more detail than this README file, a full changelog, and example use cases for Mímir!

## Inspiration

Mímir (both in concept and general architecture) is heavily inspired by [Elan Ruskin's amazing session from GDC 2012 on AI-driven Dynamic Dialog][gdc].

Fundamentally speaking, Mímir is simply a Rust implementation of Elan's proposed system for dynamic dialog. However, Mímir does offer some differences and/or extensions that cater specifically to games developed internally at Subtale (documented below).

## High-level overview

Your game's world is defined as a collection of facts: the player killed x amount of enemies, an NPC has opened y amount of doors, the player is currently near z NPC, etc.

In Mímir, facts are collected together into a map ([`Query`](#query)), where the key is the unique identifier of the fact, and the value is the fact's value (represented as a `f64`).

Also, your game will (most likey!) have predefined rules that define behaviour that should occur when one or more facts are true. We represent rules as a map ([`Rule<T>`](#rule)), where the key is the unique identifier of the fact, and the value is a predicate ([`Requirement`](#requirement)) that is evaluated against the fact's value.

Finally, rules can be stored together in collections known as rulesets ([`Ruleset<T>`](#ruleset)). Rulesets allow a query to be evaluated against many rules at once: Mímir will always look to match a query against the rule in the ruleset with the most requirements (i.e. more specific). *(If multiple rules are matched with the same specificity, one is chosen at random.)*

## Concepts

### Requirement

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

> *`RangeBound` is an enum that holds a boundary value that can be inclusive (`RangeBound::Inclusive(f64)`) or exclusive (`RangeBound::Exclusive(f64)`).*

#### Helper functions

Several helper functions are exposed to easily instantiate requirements with common equality expressions:

| Function             | Internal     | Equivalent to |
| :------------------: | :----------: | :-----------: |
| `Requirement::lt(5.)`  | `Requirement::LessThan(RangeBound::Exclusive(5.))` | `x < 5`       |
| `Requirement::lte(5.)` | `Requirement::LessThan(RangeBound::Inclusive(5.))` | `x ≤ 5`       |
| `Requirement::gt(5.)`  | `Requirement::GreaterThan(RangeBound::Exclusive(5.))`  | `x > 5`       |
| `Requirement::gte(5.)` | `Requirement::GreaterThan(RangeBound::Inclusive(5.))`  | `x ≥ 5`       |
| `Requirement::range(5., 10.)`* | `Requirement::InRange(RangeBound::Inclusive(5.), RangeBound::Exclusive(10.))` | `5 ≤ x < 10` |

*: `Requirement::range` is designed to mimic the functionality of [Python's built-in range function][py-range].

#### Real-world

In the real-world, a requirement represents a condition that must be true for a contextual event to take place. However, events will typically have many requirements that need to evaluate to true, not just one!

> For example, an NPC might query Mímir to ensure that they're only commenting on another NPC's behaviour if they've not exhibited the same behaviour previously (to avoid being hypocritical).
>
> *It could also function inversely if the NPC is intentionally a hypocrite!*

#### Floating-point equality comparison

Internally, Mímir uses the [float-cmp](https://crates.io/crates/float-cmp) crate to perform approximate comparisons between requirements and fact values when `Requirement::EqualTo` (and `Requirement::NotEqualTo`) is used.

### Query

A query is a collection of facts about the current game world's state. Mímir represents these facts in Rust as a `HashMap<FactKey, f64>`, where the `FactKey` generic type indicates the unique name of the fact, and the `f64` is the fact's value.

```rs
struct Query<FactKey> {
    facts: HashMap<FactKey, f64>,
}
```

### Rule

A `Rule` is a collection of facts and their requirements stored in a map, along with a specific outcome (`Outcome`). All requirements in the rule must evaluate to true for the rule itself to be considered true.

```rs
struct Rule<FactKey, Outcome> {
    requirements: HashMap<FactKey, Requirement>,
    pub outcome: Outcome,
}
```

#### Evaluating against queries

Rules can be evaluated against queries to determine if they are true given the current game world's state:

```rs
let mut rule = Rule::new(true);
rule.require("enemies_killed", Requirement::eq(5.));

let mut query = Query::new();
query.fact("enemies_killed", 2.5 + 1.5 + 1.);

assert!(rule.evaluate(&query));
```

In the above example, the rule evaluates to true for the supplied query because it's expecting 5 enemies to be killed (`enemies_killed`), and the query confirms the fact that 5 (`2.5 + 1.5 + 1`) have been killed.

> *Our generic outcome type (`Outcome`) for the example is just a standard boolean value (`true`). In the real-world, you'd probably use a more complex enum to denote different types of outcome (e.g. dialog, animation).*

### Ruleset

Rulesets are simply collections of rules (represented in Mímir as `Vec<Rule<FactKey, Outcome>>`).

```rs
struct Ruleset<FactKey, Outcome> {
    rules: Vec<Rule<FactKey, Outcome>>,
}
```

#### Evaluating against queries

Just like rules, rulesets can be evaluated against queries to determine if they are true given the current game world's state:

```rs
let mut rule = Rule::new("You killed 5 enemies!");
rule.require("enemies_killed", Requirement::EqualTo(5.));

let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!");
more_specific_rule.require("enemies_killed", Requirement::EqualTo(5.));
more_specific_rule.require("doors_opened", Requirement::gt(2.));

let ruleset = Ruleset::from(vec![rule, more_specific_rule]);

let mut query = Query::new();
query.fact("enemies_killed", 2.5 + 1.5 + 1.);

assert_eq!(
    ruleset.evaluate_all(&query)[0].outcome,
    "You killed 5 enemies!"
);

let mut more_specific_query = Query::new();
more_specific_query.fact("enemies_killed", 2.5 + 1.5 + 1.);
more_specific_query.fact("doors_opened", 10.);

assert_eq!(
    ruleset.evaluate_all(&more_specific_query)[0].outcome,
    "You killed 5 enemies and opened 2 doors!"
);
```

In the above example, we define a ruleset with two rules. Both rules require that 5 enemies have been killed, but one rule is more specific (also requiring that more than 2 doors have been opened).

The first query evaluates to the simpler rule, because the query does not satisfy the doors opened requirement. However, the second query evaluates to the more complex rule because the query *does* satistfy the doors opened requirement (note that even though the simpler rule is still satisfied, Mímir does not evaluate it as true because it's less specific/contains fewer requirements).

## Serialization

Requirements (including bounds), rules, and rulesets are all (de)serializable using [serde](https://serde.rs/) if you enable the respective feature in your project's `Cargo.toml`:

```toml
[dependencies]
mimir = { version = "x.x.x", features = ["serde"] }
```

This makes it easy for you to serialize rulesets into a persistent medium (i.e. files) during your game's development process, bundle them with your game, and deserialize them at runtime.

> *This also means that Mímir can effortlessly support modding by allowing you to deserialize and load user-defined rulesets at runtime.*

## Performance

> 🚧 Please note that while Mímir is in a pre-release state (<1.0.0), this section is a WIP and constantly evolving.

### Ruleset storage

Because Mímir evaluates rulesets by returning the most specific rule for a given query, the rules are stored in descending order of requirement count. This avoids scanning the entire ruleset for matching rules, as the first rules in the underlying collection are the most specific.

However, this does mean that care should be taken when invoking `ruleset.append(...)` to introduce more rules into a ruleset, as this function also triggers the underlying collection to be sorted again after the new rules are appended. *(Ideally, the rulesets should be manipulated during your game's loading state, and then only evaluated during your game's main loop.)*

### Multiple rulesets

Where possible, you should look to divide your game's entire database of rules into smaller rulesets that can be loaded in and out of memory depending on the game's current state.

For example, you might want to partition your rules into individual rulesets for each level/map/region of your game. Otherwise, you'll be subjecting yourself to an unnecessary performance cost by having Mímir evaluate rules that have no relevance to the game's current state.

> *The specific implementation of a system as described above is outside the scope of Mímir.*

## Why Mímir?

Internally at Subtale, we've adopted a naming convention around Norse mythology for tools/packages/systems and working titles for games.

Quoting from [norse-mythology.org][mimir], Mímir is an exceptionally wise being and a counselor of the gods. We hope that this contextual query engine lives up to its name in being just as wise about your game's state and defined behaviours!

*P.S. Acknowledging a happy coincidence, we love all of the work done by the folks over at [Santa Monica Studio][sms] (especially on 2018's reboot of God of War)!*

[docs]: https://mimir.subtale.com
[gdc]: https://www.youtube.com/watch?v=tAbBID3N64A
[py-range]: https://docs.python.org/3/library/functions.html#func-range
[mimir]: https://norse-mythology.org/gods-and-creatures/others/mimir/
[sms]: https://sms.playstation.com
