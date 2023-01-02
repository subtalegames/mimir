# Query

A query is a collection of facts about the current game world's state. Mímir represents these facts in Rust as a `HashMap<FactKey, f64>`, where the `FactKey` generic type indicates the unique name of the fact, and the `f64` is the fact's value.

```rs
struct Query<FactKey> {
    facts: HashMap<FactKey, f64>,
}
```