#![forbid(missing_docs)]

//! Mímir is a contextual query engine for video games with dynamic events (e.g.
//! dialog, animations) driven by their current world's state.
//!
//! At a high level, your game's world is defined as a collection of facts: the
//! player killed x amount of enemies, an NPC has opened y amount of doors, the
//! player is currently near z NPC, etc.
//!
//! In Mímir, facts are collected together into a map (`Query<FactKey,
//! FactType>`), where the key is the unique identifier of the fact, and the
//! value is the fact's value.
//!
//! Also, your game will (most likely!) have predefined rules that define
//! behaviour that should occur when one or more facts are true. We
//! represent rules as a map (`Rule<FactKey, FactType, FactEvaluator,
//! Outcome>`), where the key is the unique identifier of the fact, and the
//! value is a predicate (`Evaluator`) that is evaluated against the fact's
//! value.
//!
//! Finally, rules can be stored together in collections known as rulesets
//! (`Ruleset<FactKey, FactType, FactEvaluator, Outcome>`). Rulesets
//! allow a query to be evaluated against many rules at once: Mímir will always
//! look to match a query against the rule in the ruleset with the
//! most requirements (i.e. more specific). *(If multiple rules are matched with
//! the same specificity, one is chosen at random.)*

/// Module containing the `Evaluator` trait, used as a predicate function
/// against fact values inside rules.
pub mod evaluator;

/// Module containing a reference implementation for the `Evaluator` trait,
/// operating on `f64` values.
#[cfg(feature = "float")]
pub mod float;

/// Prelude module acting as a helper for importing Mímir into your
/// projects/crates.
pub mod prelude;

/// Module containing the `Query` struct (and accompanying logic), used to
/// represent facts (key and value) in a video game's world.
pub mod query;

/// Module containing the `Rule` struct and evaluation logic. Rules define
/// predicates (`Evaluator`) that evaluate against fact values.
pub mod rule;

/// Module containing the `Ruleset` struct (representing a collection of `Rule`
/// instances with some extra performance considerations).
pub mod ruleset;
