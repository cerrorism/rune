---
phase: 4
name: Query Layer
status: draft
last_updated: 2026-05-07
---

# Phase 4 Design — Query Layer

*(Stub — this doc will be designed in detail when Phase 3 is complete.)*

## Anticipated Rust concepts

- Generics with trait bounds (the query planner over an abstract `StorageBackend`)
- Iterator and `IntoIterator` for lazy evaluation
- Arrow columnar memory model (`RecordBatch`, `Array`, `Schema`)
- Possibly: DataFusion as a query engine integration

## Key unknowns to resolve at design time

- Build a query engine from scratch, or integrate DataFusion? (Both are valid learning choices for different reasons — decide at Phase 4 start.)
- What does "partition pruning" look like in Rune's data model?
- How does predicate pushdown interact with the `StorageBackend` trait?
- What is the simplest useful query interface — a SQL string, a builder API, or something else?
