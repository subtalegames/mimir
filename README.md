# Mímir

[![Rust](https://github.com/subtalegames/mimir/actions/workflows/rust.yml/badge.svg)][gh-workflow]

> Mímir is a contextual query engine (implemented in Rust) for video games with dynamic events (e.g. dialog, animations) driven by the current world's state.

## Documentation

Our official documentation at **[mimir.subtale.com][docs]** offers a guide with more detail than this README file, a full changelog, and example use cases for Mímir!

## Inspiration

Mímir (both in concept and general architecture) is heavily inspired by [Elan Ruskin's amazing session from GDC 2012 on AI-driven Dynamic Dialog][gdc].

Fundamentally speaking, Mímir is simply a Rust implementation of Elan's proposed system for dynamic dialog. However, Mímir does offer some differences and/or extensions that cater specifically to games developed internally at Subtale (documented below).

## High-level overview

Your game's world is defined as a collection of facts: the player killed x amount of enemies, an NPC has opened y amount of doors, the player is currently near z NPC, etc.

In Mímir, facts are collected together into a map ([`Query`][query]), where the key is the unique identifier of the fact, and the value is the fact's value (represented as a `f64`).

Also, your game will (most likey!) have predefined rules that define behaviour that should occur when one or more facts are true. We represent rules as a map ([`Rule<T>`][rule]), where the key is the unique identifier of the fact, and the value is a predicate ([`Evaluator`][evaluator]) that is evaluated against the fact's value.

Finally, rules can be stored together in collections known as rulesets ([`Ruleset<T>`][ruleset]). Rulesets allow a query to be evaluated against many rules at once: Mímir will always look to match a query against the rule in the ruleset with the most requirements (i.e. more specific). *(If multiple rules are matched with the same specificity, one is chosen at random.)*

## Example

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

## Why Mímir?

Internally at Subtale, we've adopted a naming convention around Norse mythology for tools/packages/systems and working titles for games.

Quoting from [norse-mythology.org][mimir], Mímir is an exceptionally wise being and a counselor of the gods. We hope that this contextual query engine lives up to its name in being just as wise about your game's state and defined behaviours!

*P.S. Acknowledging a happy coincidence, we love all of the work done by the folks over at [Santa Monica Studio][sms] (especially on 2018's reboot of God of War)!*

[gh-workflow]: https://github.com/subtalegames/mimir/actions/workflows/rust.yml
[docs]: https://mimir.subtale.com
[query]: https://mimir.subtale.com/concepts/query
[rule]: https://mimir.subtale.com/concepts/rule
[evaluator]: https://mimir.subtale.com/concepts/evaluator
[ruleset]: https://mimir.subtale.com/concepts/ruleset
[gdc]: https://www.youtube.com/watch?v=tAbBID3N64A
[mimir]: https://norse-mythology.org/gods-and-creatures/others/mimir/
[sms]: https://sms.playstation.com
