---
phase: 3
name: Table Format
status: draft
last_updated: 2026-05-07
---

# Phase 3 Design — Table Format

*(Stub — this doc will be designed in detail when Phase 2 is complete.)*

## Anticipated Rust concepts

- Explicit lifetime annotations (readers that hold references into metadata)
- Immutable-by-design data structures (snapshots are never mutated, only replaced)
- Builder patterns for constructing complex structs

## Inspiration

The design will be loosely inspired by Apache Iceberg's table format — immutable metadata files, snapshot history, append-only commits, simple schema evolution. Not a faithful implementation; a learning-focused simplification.

## Key unknowns to resolve at design time

- What is the minimal snapshot model? (Iceberg uses manifests + manifest lists — what does Rune need?)
- How is schema evolution represented? (Adding a nullable column is the simplest case to start with.)
- Where does manifest compaction fit, and when is it triggered?
- How do lifetimes appear when a reader holds a reference into snapshot metadata?
