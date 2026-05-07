---
last_updated: 2026-05-07
active_phase: 1
---

# Rune — Project Charter

## What this project is

A Rust learning project built around a concrete, realistic problem: a cloud-native data lake and query engine, built from scratch. The goal is not to ship a product. The goal is to deeply understand Rust — ownership, traits, async, error handling — by working on something that exercises these concepts in a way isolated toy examples never could. Along the way, the project also demystifies how real systems like Iceberg, Delta Lake, and DataFusion are designed internally.

## What this project is not

- Not a production data lake system
- Not a replacement for Iceberg, Delta Lake, Spark, or DataFusion
- Not optimized for performance (correctness and clarity come first)
- Not a complete SQL engine

## Five phases

| Phase | Name | Primary Rust concepts | Status |
|---|---|---|---|
| 1 | Local Metadata Scanner | structs, enums, traits, `Result`, `serde` | **active** |
| 2 | S3-backed Data Lake | `async/await`, `tokio`, `Arc`, ownership across await points | not started |
| 3 | Table Format | lifetimes, immutable design patterns, snapshot semantics | not started |
| 4 | Query Layer | generics, iterators, Arrow columnar memory model | not started |
| 5 | Distributed Execution | distributed systems, scheduling, fault tolerance | optional/future |

## Central architectural idea

The `StorageBackend` trait (to be defined in `src/storage/`) is the load-bearing abstraction of this entire project. All layers above Phase 1 depend on this trait, not on concrete types (`LocalStorage`, `S3Storage`). This is intentional: having two implementations (local in Phase 1, S3 in Phase 2) forces the trait to be designed honestly, and exercises Rust traits, generics, and async in a realistic setting.

## Collaboration rules (summary)

See `CLAUDE.md` for the full rules. The short version:
- Claude writes small, focused pieces; the owner reads and understands every line before it's canonical
- "Why" before "how" — the motivation for a change matters more than the mechanics
- The owner's hand-written code is the subject for review, not a target for overwriting
- No new dependencies without discussion
