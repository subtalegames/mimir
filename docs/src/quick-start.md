# Quick start <Badge type="warning" text="WIP" />

This guide serves as a tutorial for newcomers to Mímir; a hypothetical scenario is established for a game and then an exemplar, step-by-step implementation is provided.

## Scenario

You're working on an open-world game: more specifically, there's a non-linear quest tree (i.e. the player is free to explore and start/complete quests as they please).

You've decided that you'd like your game's dialogue to be more *aware* of what's happened. Specifically, you'd like NPCs to make different remarks depending on what the player has (or hasn't) already done.

This scenario can be easily achieved with Mímir and, more importantly, is exactly the kind of scenario Mímir was designed for. With this in mind, let's get started!

## Installation

Start off by adding Mímir to your project's `Cargo.toml`, including the optional `float` feature (whose relevance will be explained later):

```toml
[dependencies]
subtale-mimir = { version = "0.6", features = ["float"] }
```
