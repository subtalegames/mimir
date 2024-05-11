# Serialisation

Evaluators (including the `FloatEvaluator` implementation), rules, and rulesets are all (de)serialisable using [serde][serde] if you enable the respective feature in your project's `Cargo.toml`:

```toml
[dependencies]
subtale-mimir = { version = "0.5.1", features = ["serde"] }
```

This makes it easy for you to serialise rulesets into a persistent medium (i.e. files) during your game's development process, bundle them with your game, and deserialise them at runtime.

::: tip
This also means that Mímir can effortlessly support modding by allowing you to deserialize and load user-defined rulesets at runtime.
:::

[serde]: https://serde.rs/