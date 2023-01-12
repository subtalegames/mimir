# Mímir

[![Rust](https://github.com/subtalegames/mimir/actions/workflows/rust.yml/badge.svg)][gh-workflow]
[![Docs](https://img.shields.io/badge/docs-passing-brightgreen)][docs]
[![MIT License](https://img.shields.io/badge/license-MIT-brightgreen)][mit]
[![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-brightgreen)][apache]

> Mímir is a contextual query engine (implemented in Rust) for video games with dynamic events (e.g. dialog, animations) driven by their current world's state.

## Documentation

Our official documentation at **[mimir.subtale.com][docs]** offers a guide with more detail than this README file, a full changelog, and example use cases for Mímir!

## Inspiration

Mímir is heavily inspired (both in concept and general architecture) by [Elan Ruskin's amazing session from GDC 2012 on AI-driven Dynamic Dialog][gdc].

Fundamentally speaking, Mímir is simply a Rust implementation of Elan's proposed system for dynamic dialog. However, Mímir does offer some differences and/or extensions that cater specifically to games developed internally at Subtale (documented below).

## High-level overview

Your game's world is defined as a collection of facts: the player killed x amount of enemies, an NPC has opened y amount of doors, the player is currently near z NPC, etc.

In Mímir, facts are collected together into a map ([`Query`][query]), where the key is the unique identifier of the fact, and the value is the fact's value (represented as a `f64`).

Also, your game will (most likey!) have predefined rules that define behaviour that should occur when one or more facts are true. We represent rules as a map ([`Rule<T>`][rule]), where the key is the unique identifier of the fact, and the value is a predicate ([`Evaluator`][evaluator]) that is evaluated against the fact's value.

Finally, rules can be stored together in collections known as rulesets ([`Ruleset<T>`][ruleset]). Rulesets allow a query to be evaluated against many rules at once: Mímir will always look to match a query against the rule in the ruleset with the most requirements (i.e. more specific). *(If multiple rules are matched with the same specificity, one is chosen at random.)*

## Example

```rs
use subtale_mimir::{evaluator::{Evaluator, FloatEvaluator}, rule::{Query, Rule, Ruleset}};

// create a rule requiring that five enemies have been killed
let mut rule = Rule::new("You killed 5 enemies!");
rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));

// create a more specific rule that also requires at least 2 doors to have been opened
let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!");
more_specific_rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));
more_specific_rule.insert("doors_opened", FloatEvaluator::gte(2.));

// bundle the rules into a ruleset
let ruleset = Ruleset::new(vec![rule, more_specific_rule]);

// run a query against the ruleset
let mut query = Query::new();
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

In the above example, we define a ruleset with two rules. Both rules require that 5 enemies have been killed, but one rule is more specific (also requiring that more than 2 doors have been opened).

The first query evaluates to the simpler rule, because the query does not satisfy the doors opened requirement. However, the second query evaluates to the more complex rule because the query *does* satistfy the doors opened requirement (note that even though the simpler rule is still satisfied, Mímir does not evaluate it as true because it's less specific/contains fewer requirements).

## Libraries used

Without the following libraries, Mímir would not be where it is now:

* [float-cmp][float-cmp]: used to approximate floating-point number comparisons
* [rand][rand]: used to randomly selected evaluated rules when multiple are evaluated as true
* [serde][serde]: used to offer methods of (de)serialization
* [criterion][criterion]: used to write benchmarking test suites

## Why Mímir?

Internally at Subtale, we've adopted a naming convention around Norse mythology for tools, packages, and working titles for games.

Quoting from [norse-mythology.org][mimir]:

> Mímir is an exceptionally wise being and a counselor of the gods.

We hope that this contextual query engine lives up to its name in being just as wise about your game's state and defined behaviours!

*P.S. Acknowledging a happy coincidence in the naming of Mímir, we love all of the work done by the folks over at [Santa Monica Studio][sms] (especially on 2018's reboot of God of War)!*

## License

Mímir is free and open source. Unless explicitly noted otherwise, all code in this repository is dual-licensed under the [MIT License][mit] and [Apache License, Version 2.0][apache].

This licensing approach is the de facto standard within the Rust ecosystem.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[gh-workflow]: https://github.com/subtalegames/mimir/actions/workflows/rust.yml
[docs]: https://mimir.subtale.com
[query]: https://mimir.subtale.com/concepts/query
[rule]: https://mimir.subtale.com/concepts/rule
[evaluator]: https://mimir.subtale.com/concepts/evaluator
[ruleset]: https://mimir.subtale.com/concepts/ruleset
[gdc]: https://www.youtube.com/watch?v=tAbBID3N64A
[mimir]: https://norse-mythology.org/gods-and-creatures/others/mimir/
[sms]: https://sms.playstation.com
[mit]: LICENSE-MIT
[apache]: LICENSE-APACHE
[float-cmp]: https://github.com/mikedilger/float-cmp
[rand]: https://github.com/rust-random/rand
[serde]: https://github.com/serde-rs/serde
[criterion]: https://github.com/bheisler/criterion.rs
