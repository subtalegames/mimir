---
editLink: false
outline: 3
---

# Changelog

::: tip
Visit the [releases page on GitHub](https://github.com/subtalegames/mimir/releases) for all historical releases.
:::

### [0.3.0](https://github.com/subtalegames/mimir/releases/tag/v0.3.0)

Refactored to use generics for fact identifiers/names (`Query` => `Query<FactKey>`, `Rule<T>` => `Rule<FactKey, Outcome>`, `Ruleset<T>` => `Ruleset<FactKey, Outcome>`).

### [0.2.0](https://github.com/subtalegames/mimir/releases/tag/v0.2.0)

Introduced `Criterion::NotEqualTo(f64)` for defining criteria for facts that don't equal a supplied floating-point number.

### [0.1.0](https://github.com/subtalegames/mimir/releases/tag/v0.1.0)

Initial pre-release of Mímir.
