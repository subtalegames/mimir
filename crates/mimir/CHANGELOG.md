# Changelog

## 0.3.0 (2022-16-12)

Refactored to use generics for fact identifiers/names (`Query` => `Query<FactKey>`, `Rule<T>` => `Rule<FactKey, Outcome>`, `Ruleset<T>` => `Ruleset<FactKey, Outcome>`).

## 0.2.0 (2022-12-12)

Introduced `Criterion::NotEqualTo(f64)` for defining criteria for facts that don't equal a supplied floating-point number.

## 0.1.0 (2022-12-12)

Initial pre-release of Mímir.
