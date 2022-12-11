# Mimir

> 🧠 Contextual query engine for dynamic video games

Mimir is a contextual query engine (implemented in Rust) for video games with dynamic "events" (e.g. dialog, animations) driven by the current world's "state" (context).

## Inspiration

Mimir (both in concept and general architecture) is heavily inspired by [Elan Ruskin's amazing session from GDC 2012 on AI-driven Dynamic Dialog][gdc].

At a high-level, Mimir is simply a Rust implementation of Elan's proposed system for dynamic dialog. However, Mimir does offer some differences and/or extensions that cater specifically to games developed internally at Subtale (documented below).

## Concepts

### Criterion

A `Criterion` is simply a definition of a predicate on a double precision floating-point number (represented in Mimir using Rust's `f64` type).

```rs
struct Criterion {
  gt: f64,
  gt_inclusive: bool,
  lt: f64,
  lt_inclusive: bool,
}
```

> *`f64::INFINITY` and `f64::NEG_INFINITY` must be used to indicate criterion with no upper or lower bound (respectively).*
>
> *`gt_inclusive` and `lt_inclusive` must be used to indicate criterion with inclusive upper or lower bounds (respectively).*

#### Helper functions

Several helper functions are exposed to easily instantiate criterion with common equality expressions:

| Function             | Internal     | Equivalent to |
| :------------------: | :----------: | :-----------: |
| `Criterion::eq(5.)`  | `5 ≤ x ≤ 5`  | `x = 5`       |
| `Criterion::lt(5.)`  | `-∞ ≤ x < 5` | `x < 5`       |
| `Criterion::lte(5.)` | `-∞ ≤ x ≤ 5` | `x ≤ 5`       |
| `Criterion::gt(5.)`  | `5 < x ≤ ∞`  | `x > 5`       |
| `Criterion::gte(5.)` | `5 ≤ x ≤ ∞`  | `x ≥ 5`       |

[gdc]: https://www.youtube.com/watch?v=tAbBID3N64A
