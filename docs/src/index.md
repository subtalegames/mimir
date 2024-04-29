---
layout: home

hero:
  name: "Mímir"
  text: "Contextual query engine for dynamic video games."
  tagline: "Game logic expressed as queryable rules."
  image:
    src: /hero.svg
    alt: Brain illustration
  actions:
    - theme: brand
      text: Get Started
      link: /markdown-examples
    - theme: brand
      text: API Reference
      link: https://docs.rs/subtale-mimir
    - theme: alt
      text: More Subtale Open Source
      link: https://github.com/subtalegames

features:
  - icon: 📋
    title: Rule-based events
    details: Compose rules from predicates that trigger outcomes in your game's world.
  - icon: ⚡️
    title: Optimised ruleset storage
    details: Process groups of rules efficiently, avoiding unnecessary evaluations.
  - icon: 🛟
    title: Compile-time safety
    details: Catch errors in your game's ruleset at compile-time, not runtime.
---

## Quickstart

Add `subtale-mimir = "0.6"` to your `Cargo.toml` file's `[dependencies]` section, then declare your game's first ruleset:

```rust
use subtale_mimir::prelude::*;

fn main() {
    let mut rule = Rule::new("You killed 5 enemies!");
    rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));

    let mut more_specific_rule = Rule::new("You killed 5 enemies and opened 2 doors!");
    more_specific_rule.insert("enemies_killed", FloatEvaluator::EqualTo(5.));
    more_specific_rule.insert("doors_opened", FloatEvaluator::gte(2.));

    let ruleset = Ruleset::new(vec![rule, more_specific_rule]);

    let mut query = Query::new();
    query.insert("enemies_killed", 2.5 + 1.5 + 1.);

    assert_eq!(
        ruleset.evaluate(&query).first().unwrap().outcome,
        "You killed 5 enemies!"
    );

    let mut more_specific_query = Query::new();
    more_specific_query.insert("enemies_killed", 2.5 + 1.5 + 1.);
    more_specific_query.insert("doors_opened", 10.);

    assert_eq!(
        ruleset.evaluate(&more_specific_query).first().unwrap().outcome,
        "You killed 5 enemies and opened 2 doors!"
    );
}
```
