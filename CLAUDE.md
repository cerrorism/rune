# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Purpose

Rune is a Rust learning project. The owner is building a data lake from scratch — not to ship a product, but to deeply understand Rust and how cloud-native analytical systems work internally (Iceberg, Delta Lake, DataFusion, etc.). **Learning and clarity are the primary goals; production readiness is not.**

## How to Collaborate

- **Start each session by reading `docs/README.md`** — it tells you exactly which files to read to restore context, and which to update before stopping.
- **Write small, focused pieces of code.** Each addition should be digestible — the owner needs to read and understand every line before it becomes canonical.
- **Explain the "why" before the "how."** When suggesting a change, lead with the motivation (what Rust concept it exercises, what problem it avoids, what trade-off it resolves). Don't just show a diff.
- **Treat the owner's hand-written code as the subject for review**, not as something to overwrite. When reviewing, identify the most important issue first and explain the principle behind it.
- **Do not introduce abstractions preemptively.** Wait until there are at least two concrete use cases before generalizing. A duplicated `match` is better than a premature trait.
- **Don't add dependencies without discussion.** The `[dependencies]` table is intentionally empty; each crate added should be a deliberate learning decision.

## Development Commands

```bash
cargo build          # compile
cargo run            # run binary
cargo test           # all tests
cargo test <name>    # single test by name substring
cargo fmt            # format (run before committing)
cargo clippy         # lint
```

## Planned Architecture

The project grows in phases; only Phase 1 is active. Future modules live under `src/`:

```
storage/    # StorageBackend trait; local.rs first, s3.rs later (Phase 2)
metadata/   # manifest.rs (JSON manifests), snapshot.rs (Phase 3)
table/      # schema.rs, partition.rs (Phase 3)
query/      # planner.rs, executor.rs (Phase 4)
formats/    # parquet.rs, arrow integration (Phase 4)
```

The `StorageBackend` trait in `storage/` is the central abstraction — it hides whether data lives on disk or in S3. Every higher layer depends on this trait, not on concrete types. This is where Rust traits, generics, and async (`async_trait`) will be exercised most heavily.

## Key Rust Concepts by Phase

| Phase | Primary Rust Concepts |
|---|---|
| 1 — Local scanner | structs, enums, traits, `Result`, `serde` |
| 2 — S3 backend | `async/await`, `tokio`, `Arc`, ownership across await points |
| 3 — Table format | lifetime annotations, immutable design patterns |
| 4 — Query engine | generics, iterators, `Arrow` columnar memory model |

## Conventions

- Error handling: prefer `thiserror` for library-style errors, `anyhow` for binary/CLI code. Don't use `.unwrap()` outside of tests.
- No `unsafe` without an explicit discussion of why it's necessary.
- Module files use `mod.rs` only when a module has sub-modules; otherwise use the flat `module_name.rs` style.
