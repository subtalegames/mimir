# Repeated evaluations

As you start to delve into Mímir, you'll probably come across a scenario where you only want a rule to be evaluated once (or `n` amount of times).

There a few different solutions to achieve this functionality, which we'll explore on this page.

## Removing from the ruleset

The naive approach would be to mutate the ruleset after evaluation to remove the rule that shouldn't be repeated. However, this approach has multiple drawbacks (and we **don't** recommend that you go down this route in your implementation).

Firstly, as explained on the [performance page](/performance.html#ruleset-storage), creating (and modifying) a ruleset is expensive, because it needs to be (re)sorted in such a way that evaluations are more performant.

Also, as described on the [serialization page](/serialization.html), we recommend that your implementation uses serialized rulesets that are bundled as assets alongside your game's executable and then deserialized at runtime. By introducing the logic of removing rules after evaluation, you will also need to re-serialize your ruleset and overwrite your persistent assets.

## Storing evaluation history

We recommend that you track which rules are evaluated by Mímir (and potentially how many times they are evaluated) and store this data alongisde the rest of your game's persistent state (e.g. a save file).

With this tracking system established, you can add evaluators to your rules that check if the rule hasn't been evaluated before.

You'll need to implement the logic that populates every query made during evaluations with the history of what rules have been evaluated.
