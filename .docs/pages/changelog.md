# Changelog

> Visit the [releases page on GitHub](https://github.com/subtalegames/mimir/releases) for a list of all historical releases.

### [v0.3.0](https://github.com/subtalegames/mimir/releases/tag/v0.3.0) (2022-12-16)

Refactored to use generics for fact identifiers/names (`Query` => `Query<FactKey>`, `Rule<T>` => `Rule<FactKey, Outcome>`, `Ruleset<T>` => `Ruleset<FactKey, Outcome>`).

### [v0.2.0](https://github.com/subtalegames/mimir/releases/tag/v0.2.0) (2022-12-12)

Introduced `Criterion::NotEqualTo(f64)` for defining criteria for facts that don't equal a supplied floating-point number.

### [v0.1.0](https://github.com/subtalegames/mimir/releases/tag/v0.1.0) (2022-12-12)

Initial pre-release of Mímir.
