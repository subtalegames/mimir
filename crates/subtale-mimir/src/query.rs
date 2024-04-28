use indexmap::IndexMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Query<FactKey, FactType>` represents a collection of facts about your
/// video game's world, mapped from a fact's unique identifier (`FactKey`) to
/// current value (`FactType`).
///
/// The logic for obtaining an instance of `Query` is outside the scope of
/// Mímir: your game should already have systems in place that track state and
/// various facts about your world.
///
/// ```
/// use subtale_mimir::prelude::*;
///
/// let mut query: Query<&str, usize> = Query::new(); // You can use ::with_capacity(usize) if applicable!
/// query.insert("enemies_killed", 5);
/// query.insert("current_level", 2);
/// ```
///
/// In reality, you will most likely use an enum for the `FactType` generic so
/// you can store varying types in your query:
///
/// ```
/// use subtale_mimir::prelude::*;
///
/// #[derive(Clone, Copy)]
/// enum QueryValue {
///     Int(usize),
///     Decimal(f64),
///     Flag(bool),
/// }
///
/// let mut query: Query<&str, QueryValue> = Query::new();
/// query.insert("enemies_killed", QueryValue::Int(5));
/// query.insert("player_health", QueryValue::Decimal(12.34));
/// query.insert("reached_checkpoint", QueryValue::Flag(false));
/// ```
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Query<FactKey, FactType>
where
    FactKey: std::hash::Hash + Eq,
{
    /// The facts currently stored within the query (using an `IndexMap` as the
    /// data structure implementation).
    pub facts: IndexMap<FactKey, FactType>,
}

impl<FactKey: std::hash::Hash + Eq, FactType: Copy> Query<FactKey, FactType> {
    /// Instantiates a new instance of `Query` without allocating an underlying
    /// `IndexMap`.
    ///
    /// Computes in `O(1)` time.
    pub fn new() -> Self {
        Self {
            facts: IndexMap::new(),
        }
    }

    /// Instantiates a new instance of `Query` with a pre-allocated underlying
    /// `IndexMap` (unless `capacity` is zero).
    ///
    /// Computes in `O(n)` time.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            facts: IndexMap::with_capacity(capacity),
        }
    }

    /// Inserts a new fact into the query (at the end of the query's underlying
    /// map).
    ///
    /// If a fact already exists in the query with the same key, the value is
    /// overwritten (but the index position of the fact in the query is
    /// retained).
    ///
    /// Computes in `O(1)` time (amortized average, depending on current
    /// capacity).
    pub fn insert(&mut self, fact: FactKey, value: FactType) { self.facts.insert(fact, value); }

    /// Appends all facts from another query to the query (at the end of the
    /// query's underlying map).
    ///
    /// Computes in `O(1)` time.
    pub fn extend(&mut self, query: Query<FactKey, FactType>) { self.facts.extend(query.facts); }
}

#[cfg(test)]
mod tests {
    use super::Query;

    #[test]
    fn new_query() {
        let query: Query<&str, i32> = Query::new();
        assert_eq!(query.facts.len(), 0);
    }

    #[test]
    fn new_query_with_capacity() {
        let query: Query<&str, i32> = Query::with_capacity(10);
        assert_eq!(query.facts.len(), 0);
    }

    #[test]
    fn query_insertion() {
        let mut query = Query::new();
        query.insert("fact1", 1);
        query.insert("fact2", 2);

        assert_eq!(query.facts.len(), 2);
        assert_eq!(query.facts.get("fact1"), Some(&1));
        assert_eq!(query.facts.get("fact2"), Some(&2));
    }

    #[test]
    fn query_override_insertion() {
        let mut query = Query::new();
        query.insert("fact1", 1);
        query.insert("fact1", 2);

        assert_eq!(query.facts.len(), 1);
        assert_eq!(query.facts.get("fact1"), Some(&2));
    }

    #[test]
    fn query_extension() {
        let mut query1 = Query::new();
        query1.insert("fact1", 1);
        query1.insert("fact2", 2);

        let mut query2 = Query::new();
        query2.insert("fact3", 3);
        query2.insert("fact4", 4);

        query1.extend(query2);

        assert_eq!(query1.facts.len(), 4);
        assert_eq!(query1.facts.get("fact1"), Some(&1));
        assert_eq!(query1.facts.get("fact2"), Some(&2));
        assert_eq!(query1.facts.get("fact3"), Some(&3));
        assert_eq!(query1.facts.get("fact4"), Some(&4));
    }
}
