# Changelog

Visit the [releases page on GitHub][releases] for a list of all historical releases.

## [v0.4.0](https://github.com/subtalegames/mimir/releases/tag/v0.4.0) (2023-02-27)

* Migrated library to Cargo workspace for future modularity
* Replaced usage of BTreeMap with IndexMap
* **BREAKING:** Renamed `Criterion` to `Requirement`
* Created initial version of this website
* Added example use case to website (loading screen tips)
* Reworked structs to support generic fact value type
* Implemented trait-based system for evaluating requirements
* **BREAKING:** Renamed `Ruleset::from` to `Ruleset::new`
* **BREAKING:** Renamed many functions names to be more "idiomatic" (mirroring Rust's standard library)
* Renamed Cargo.toml crate name from `mimir` to `subtale-mimir` (due to crates.io clash)
* Added justfile for dev/build tasks
* Added Subtale's opinionated rustfmt configuration
* Migrated documentation site from JavaScript to TypeScript
* Added check against query length during rule evaluation (for performance)

## [v0.3.0](https://github.com/subtalegames/mimir/releases/tag/v0.3.0) (2022-12-16)

Refactored to use generics for fact identifiers/names (`Query` => `Query<FactKey>`, `Rule<T>` => `Rule<FactKey, Outcome>`, `Ruleset<T>` => `Ruleset<FactKey, Outcome>`).

## [v0.2.0](https://github.com/subtalegames/mimir/releases/tag/v0.2.0) (2022-12-12)

Introduced `Criterion::NotEqualTo(f64)` for defining criteria for facts that don't equal a supplied floating-point number.

## [v0.1.0](https://github.com/subtalegames/mimir/releases/tag/v0.1.0) (2022-12-12)

Initial pre-release of Mímir.

[releases]: https://github.com/subtalegames/mimir/releases
