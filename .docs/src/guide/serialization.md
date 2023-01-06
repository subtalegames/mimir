# Serialization

Evaluators (including the `FloatEvaluator` implementation), rules, and rulesets are all (de)serializable using [serde][serde] if you enable the respective feature in your project's `Cargo.toml`:

```toml
[dependencies]
mimir = { version = "x.x.x", features = ["serde"] }
```

This makes it easy for you to serialize rulesets into a persistent medium (i.e. files) during your game's development process, bundle them with your game, and deserialize them at runtime.

::: info
This also means that Mímir can effortlessly support modding by allowing you to deserialize and load user-defined rulesets at runtime.
:::

[serde]: https://serde.rs/