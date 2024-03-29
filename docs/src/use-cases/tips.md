# Loading screen tips

For as long as most gamers can remember, video games have offered tips and hints on loading screens (and various other game states inbetween gameplay, e.g. lobbies).

This page describes an example implementation of Mímir providing hints that dynamically change based on game events that have recently happened (or are about to happen).

## Outcome type

In this use case, our outcome type will be an enum (`Outcome`) with one variant (`Tip`) that contains the tip's message (`String`) and the ID (`usize`, for simplicity) of the model that we want to render alongside the tip (à la Bethesda titles):

```rs
enum Outcome {
    Tip { message: String, model_id: usize },
}
```

## Defining some tips

Now let's define some tips (represented as [rules](/concepts/rule.html) in Mímir):

```rs
use subtale_mimir::prelude::*;

let mut just_died = Rule::new(Outcome::Tip {
    message: "Have you tried, like, not dying?".into(),
    model_id: 123,
});

let mut finished_level_three = Rule::new(Outcome::Tip {
    message: "Thought that was hard? You ain't seen nothing yet...".into(),
    model_id: 456,
});
```

> ℹ️ In a production environment (i.e. distributing your game), it makes more sense to serialize your tips during development and include them in your distributed assets, ready to be [deserialized at runtime](/serialization.html)!

### Adding requirements

Without [evaluators](/concepts/evaluator.html) (requirements), these tips are pretty useless. Let's add some!

> ⚠️ The `FloatEvaluator` implementation used in this example requires enabling the `float` feature in your project's `Cargo.toml`!

```rs
just_died.insert(
    "player_just_died",
    FloatEvaluator::EqualTo(1.),
);

finished_level_three.insert(
    "last_level_completed",
    FloatEvaluator::EqualTo(3.),
);

// Your game needs to maintain the values of `player_just_died` and
// `last_level_completed`: this logic is outside of Mímir's scope!
```

> ℹ️ In the above example, we mimick a `bool` by checking if the float's value is equal to `1.0` (`FloatEvaluator::EqualTo(1.)`).
>
> Alternatively, you could write your own implementation of `Evaluator` that can evaluate boolean values.

## Bundling the tips

Now let's bundle the tips into a [ruleset](/concepts/ruleset.html) so we can evaluate them all against our game's current state in a performant manner:

```rs
let tips = Ruleset::new(vec![just_died, finished_level_three]);
```

> ⚠️ As outlined on the [performance page](/performance.html#ruleset-storage), invoking `Ruleset::new` is expensive!
>
> Instead of creating the ruleset each time your game enters a loading screen state, you should setup your ruleset once during your game's initial load.

## Retrieving a valid tip

Now that are tips are stored in a rulset, we can evaluate the ruleset (supplying our game's current state as a [query](/concepts/query.html)) and see if any of our tips are applicable!

```rs
let mut current_state = Query::new();
query.insert("player_just_died", 1.);
query.insert("last_level_completed", 4.);

let tip = tips.evaluate(&current_state);
// Some(Rule { outcome: { message: "Have you tried, like, not dying?" }})
```

### No valid tips

You may find that there are no valid tips based on what you've defined and your game's current state; this is completely normal behaviour!

`Ruleset::evaluate` returns an `Option<Rule<...>`: this means that you can use a match expression and perform some alternative logic if no matching tip is found (i.e. pick from a selection of predefined, generic tips).

### Multiple valid tips

It's also a reasonable expectation that there will be times when your game's current state matches multiple tips that you've defined.

Out-of-the-box, Mímir will evaluate to a randomly chosen rule in a ruleset if multiple are evaluated as true.

Alternatively, you can use `Ruleset::evaluate_all(&current_state)` to return a `Vec<Rule<...>>`: this means that you can iterate over all of the rules that Mímir evaluated as true (none, one, or many).
