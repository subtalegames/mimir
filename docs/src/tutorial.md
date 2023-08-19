# Tutorial (WIP)

This guide serves as a tutorial for newcomers to Mímir; a hypothetical scenario is established for a game and then an exemplar, step-by-step implementation is provided.

## Scenario

You're working on a game that is an "open world": more specifically, there is a non-linear quest tree (i.e. the player is free to explore and start/complete quests as they please).

As a game designer, you've decided that you'd like your game's dialogue to be more "aware" of what has happened. You'd like NPCs to make different remarks depending on what the player has/hasn't already done.

This scenario can be easily achieved with Mímir; let's get started!

## Steps

### Installation

Start off by adding Mímir to your project's `Cargo.toml`, including the optional `float` feature (whose relevance will be explained later):

```toml
[dependencies]
subtale-mimir = { version = "0.5.1", features = ["float"] }
```
