# High-level overview

## Queries

Your game's world is defined as a collection of facts: the player killed x amount of enemies, an NPC has opened y amount of doors, the player is currently near z NPC, etc.

In Mímir, facts are collected together into a map ([`Query`](/concepts/query.html)), where the key is the unique identifier of the fact, and the value is the fact's value.

## Rules and evaluators

Also, your game will (most likey!) have predefined rules that define behaviour that should occur when one or more facts are true. We represent rules as a map ([`Rule`](/concepts/rule.html)), where the key is the unique identifier of the fact, and the value is a predicate ([`Evaluator`](/concepts/evaluator.html)) that is evaluated against the fact's value.

## Rulesets

Finally, rules can be stored together in collections known as rulesets ([`Ruleset`](/concepts/ruleset.html)). Rulesets allow a query to be evaluated against many rules at once: Mímir will always look to match a query against the rule in the ruleset with the most requirements (i.e. more specific).

> ℹ️ If multiple rules with the same specificity are matched within a ruleset, one is chosen at random.
