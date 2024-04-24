![Mimir GitHub banner](https://github.com/subtalegames/mimir/assets/24438483/726885b3-98e4-47bc-b8f3-ae02653f6041)

[![OSS by Subtale](https://img.shields.io/badge/oss_by-subtale-f0f0f1?style=flat-square&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyBpZD0iTGF5ZXJfMiIgZGF0YS1uYW1lPSJMYXllciAyIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI3MTQuOTciIGhlaWdodD0iNDYxLjEzIiB2aWV3Qm94PSIwIDAgNzE0Ljk3IDQ2MS4xMyI%2BCiAgPGcgaWQ9IkxheWVyXzEtMiIgZGF0YS1uYW1lPSJMYXllciAxIj4KICAgIDxnPgogICAgICA8cGF0aCBkPSJNMzU3LjQ4LDM1Ny41MWgtMjIyLjc4Yy05Ljk1LDAtMTkuNS0zLjk1LTI2LjU0LTEwLjk5TDAsMjM4LjM1aDIzOC4zMmwxMTkuMTYsMTE5LjE2WiIgZmlsbD0iI2ZmZiIgc3Ryb2tlLXdpZHRoPSIwIi8%2BCiAgICAgIDxwYXRoIGQ9Ik03MTQuOTcsMjM4LjM1bC0yMTEuNzgsMjExLjc4Yy0xNC42NiwxNC42Ni0zOC40MiwxNC42Ni01My4wOCwwbC05Mi42Mi05Mi42MiwxMTkuMTYtMTE5LjE2aDIzOC4zMloiIGZpbGw9IiNmZmYiIHN0cm9rZS13aWR0aD0iMCIvPgogICAgICA8cGF0aCBkPSJNNDc2LjY3LDExMC43M3YxNi45MWMtMjguMzEsMC01Ni42OCwxMC44Mi03OC4zMiwzMi40Ny0yMS42NSwyMS41OC0zMi40MSw0OS44OS0zMi40MSw3OC4yNmgtMTYuOTFjMC0yOC4zMS0xMC44Mi01Ni42OC0zMi40Ny03OC4zMi0yMS41OC0yMS42NS00OS44OS0zMi40LTc4LjI2LTMyLjR2LTE2LjkxYzI4LjMxLDAsNTYuNjgtMTAuODIsNzguMzItMzIuNDcsMjEuNjUtMjEuNTgsMzIuNDEtNDkuODksMzIuNDEtNzguMjZoMTYuOTFjMCwyOC4zMSwxMC44Miw1Ni42OCwzMi40Nyw3OC4zMiwyMS41OCwyMS42NSw0OS44OSwzMi40LDc4LjI2LDMyLjRaIiBmaWxsPSIjZmZmIiBzdHJva2Utd2lkdGg9IjAiLz4KICAgIDwvZz4KICA8L2c%2BCjwvc3ZnPg%3D%3D&logoColor=f0f0f1&labelColor=2060d3)][oss]
[![Chat on Discord](https://img.shields.io/badge/chat_on-discord-f0f0f1?style=flat-square&logo=discord&logoColor=f0f0f1&labelColor=2060d3)][discord]
[![Rust](https://img.shields.io/github/actions/workflow/status/subtalegames/mimir/rust.yml?style=flat-square&color=f0f0f1&logo=github-actions&logoColor=f0f0f1&labelColor=2060d3)][gh-workflow]
[![Docs](https://img.shields.io/badge/docs-passing-brightgreen?style=flat-square&color=f0f0f1&logo=github-actions&logoColor=f0f0f1&labelColor=2060d3)][docs]
[![MIT License](https://img.shields.io/badge/license-MIT-f0f0f1?style=flat-square&labelColor=2060d3)][mit]
[![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-f0f0f1?style=flat-square&labelColor=2060d3)][apache]

> Mimir is a contextual query engine (implemented in Rust) for video games with dynamic events (e.g. dialogue, animations) driven by their current world's state.

## Documentation

Our official documentation at **[mimir.subtale.com][docs]** offers significantly more detail than this README, including:

* [a tutorial for newcomers][tutorial]
* [a full changelog][changelog]
* [example use cases][use-cases]

## Inspiration

Mímir is heavily inspired (both in concept and general architecture) by [Elan Ruskin's amazing session from GDC 2012 on AI-driven Dynamic Dialog][gdc].

Fundamentally speaking, Mímir is simply a Rust implementation of Elan's proposed system for dynamic dialog. However, Mímir does offer some differences and/or extensions that cater specifically to games developed internally at Subtale (documented below).

## High-level overview

Your game's world is defined as a collection of facts: the player killed x amount of enemies, an NPC has opened y amount of doors, the player is currently near z NPC, etc.

In Mímir, facts are collected together into a map ([`Query<FactKey, FactType>`][query]), where the key is the unique identifier of the fact, and the value is the fact's value.

Also, your game will (most likely!) have predefined rules that define behaviour that should occur when one or more facts are true. We represent rules as a map ([`Rule<FactKey, FactType, FactEvaluator, Outcome>`][rule]), where the key is the unique identifier of the fact, and the value is a predicate ([`Evaluator`][evaluator]) that is evaluated against the fact's value.

Finally, rules can be stored together in collections known as rulesets ([`Ruleset<FactKey, FactType, FactEvaluator, Outcome>`][ruleset]). Rulesets allow a query to be evaluated against many rules simultaneously: Mímir will always look to match a query against the rule in the ruleset with the most requirements (i.e. more specific). *(If multiple rules are matched with the same specificity, one is chosen at random.)*

## Example

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

In the above example, we define a ruleset with two rules. Both rules require that 5 enemies have been killed, but one rule is more specific (also requiring that more than 2 doors have been opened).

The first query evaluates to the simpler rule because the query does not satisfy the doors opened requirement. However, the second query evaluates to the more complex rule because the query *does* satisfy the doors opened requirement (note that even though the simpler rule is still satisfied, Mímir does not evaluate it as true because it's less specific/contains fewer requirements).

## Libraries used

Without the following libraries, Mímir would not be where it is now:

* [float-cmp][float-cmp]: used to approximate floating-point number comparisons
* [indexmap][indexmap]: used as the implementation of underlying map structures
* [rand][rand]: used to randomly select evaluated rules when multiple are evaluated as true
* [serde][serde]: used to offer methods of (de)serialization
* [criterion][criterion]: used to write benchmarking test suites

## License

Mímir is free and open source. Unless explicitly noted otherwise, all code in this repository is dual-licensed under the [MIT License][mit] and [Apache License, Version 2.0][apache].

This licensing approach is the de facto standard within the Rust ecosystem.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[oss]: https://subtale.dev
[discord]: https://discord.subtale.com
[gh-workflow]: https://github.com/subtalegames/mimir/actions/workflows/rust.yml
[docs]: https://mimir.subtale.com
[tutorial]: https://mimir.subtale.com/tutorial.html
[changelog]: https://mimir.subtale.com/changelog.html
[use-cases]: https://mimir.subtale.com/use-cases/tips.html
[query]: https://mimir.subtale.com/concepts/query.html
[rule]: https://mimir.subtale.com/concepts/rule.html
[evaluator]: https://mimir.subtale.com/concepts/evaluator.html
[ruleset]: https://mimir.subtale.com/concepts/ruleset.html
[gdc]: https://www.youtube.com/watch?v=tAbBID3N64A
[mit]: LICENSE-MIT
[apache]: LICENSE-APACHE
[float-cmp]: https://github.com/mikedilger/float-cmp
[indexmap]: https://github.com/bluss/indexmap
[rand]: https://github.com/rust-random/rand
[serde]: https://github.com/serde-rs/serde
[criterion]: https://github.com/bheisler/criterion.rs
