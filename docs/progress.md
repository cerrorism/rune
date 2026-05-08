---
last_updated: 2026-05-07
active_phase: 1
last_session_summary: "Added a small Parquet reader/writer wrapper around the official parquet crate using Arrow RecordBatches."
---

# Rune — Progress Tracker

## Current focus

**Phase 1 — Local Metadata Scanner**
Next task: Design `FileEntry` struct and write `src/metadata/manifest.rs` with serde serialization.

---

## Phase 1 — Local Metadata Scanner

### Setup
- [x] Rust environment (WSL2, stable toolchain, RustRover)
- [x] `cargo new rune` — project initialized
- [x] `CLAUDE.md` created
- [x] `docs/` folder structure created
- [x] `cargo fmt` and `cargo clippy` are clean

### Core implementation
- [x] `src/formats/parquet.rs` — first reader/writer wrapper using the official `parquet` crate
- [ ] Define `FileEntry` struct in `src/metadata/manifest.rs`
- [ ] Define `FileType` enum (`Parquet`, `Csv`, `Unknown`)
- [ ] Partition key extraction from path segments (`year=2025/month=05`)
- [ ] `src/storage/local.rs` — walk directory tree, collect file paths
- [ ] Wire together in `main.rs`: scan path → build manifest → write JSON

### Tests
- [x] Unit test: Parquet `RecordBatch` write/read round-trip
- [ ] Unit test: partition key parsing from a path string
- [ ] Unit test: `FileEntry` round-trips through serde JSON
- [ ] Integration test: scan a `tests/fixtures/` directory, verify manifest output

### Learning milestones
- [ ] First `#[derive(Debug, Serialize, Deserialize)]`
- [x] First custom error type with `thiserror`
- [x] First use of `?` operator propagating across error types
- [ ] First `impl Trait for Struct`
- [ ] First meaningful use of `Iterator` combinators (`filter_map`, `collect`)

---

## Phase 2 — S3-backed Data Lake
*(locked — begins after Phase 1 complete)*

- [ ] Add `tokio` as first async dependency (with discussion)
- [ ] Define `async` methods on `StorageBackend` trait
- [ ] `src/storage/s3.rs` using `aws-sdk-s3`
- [ ] Parallel file listing with `tokio::spawn`
- [ ] Incremental snapshot tracking

### Learning milestones
- [ ] First `async fn` and `.await`
- [ ] First `Arc<T>` for shared ownership across tasks
- [ ] First `async_trait` on a trait definition

---

## Phase 3 — Table Format
*(locked)*

- [ ] Table metadata JSON schema
- [ ] Snapshot history (append-only commits)
- [ ] Simple schema evolution
- [ ] Manifest compaction

### Learning milestones
- [ ] First use of explicit lifetime annotations
- [ ] Understanding immutable-by-design data structures

---

## Phase 4 — Query Layer
*(locked)*

- [ ] Add Arrow dependency
- [ ] Partition pruning
- [ ] Projection pushdown
- [ ] Local execution engine

### Learning milestones
- [ ] Understanding Arrow columnar memory model
- [ ] First use of higher-order generics

---

## Phase 5 — Distributed Execution
*(locked — optional)*

- [ ] Worker node protocol
- [ ] Distributed scan execution
- [ ] Simple scheduler

---

## Completed sessions log

| Date | What was done |
|---|---|
| 2026-05-07 | Added initial Parquet read/write wrapper using `parquet`, `arrow-array`, and `arrow-schema`; tests and clippy pass |
| 2026-05-07 | Project initialized; `CLAUDE.md`, `README.md`, and `docs/` folder structure created |
