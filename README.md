# Rune

A personal Rust learning project focused on building a lightweight cloud-native data lake and query engine from scratch.

The goal of this project is **not** to compete with production systems like Apache Iceberg, Delta Lake, Spark, or DataFusion. Instead, the project is designed as a hands-on way to deeply learn:

- Rust language and ownership model
- Async programming with Tokio
- Cloud-native storage patterns
- Columnar data formats (Parquet / Arrow)
- Metadata-driven table formats
- Query planning and execution
- Distributed systems concepts
- Performance-oriented systems programming

This project intentionally starts small and evolves incrementally.

---

# Philosophy

Modern data systems are built from a surprisingly small set of core ideas:

- immutable files
- metadata manifests
- partition pruning
- schema evolution
- snapshot/version tracking
- vectorized execution
- lazy query planning
- object storage semantics

Instead of treating systems like Iceberg or Spark as “magic,” this project explores how these concepts actually work internally.

The project is also an experiment in learning Rust through a realistic systems project rather than isolated toy examples.

---

# Initial Goals

## Phase 1 — Local Metadata Scanner

Learn:
- Rust basics
- File IO
- Structs/enums/traits
- Error handling
- Serde serialization

Features:
- Scan local directories
- Detect Parquet/CSV files
- Infer partitions from directory structure
- Generate metadata manifests

Example:

```text
data/
  year=2025/month=05/day=01/file.parquet
```

Produces:

```json
{
  "partitions": {
    "year": "2025",
    "month": "05",
    "day": "01"
  }
}
```

---

## Phase 2 — S3-backed Data Lake

Learn:
- Async Rust
- Tokio runtime
- AWS SDK for Rust
- Streaming IO
- Concurrency patterns

Features:
- Read/write metadata to S3
- Incremental snapshot tracking
- Parallel file listing
- Content-addressed manifests

---

## Phase 3 — Table Format

Learn:
- Immutable metadata design
- Snapshot/version semantics
- Schema handling
- Manifest compaction

Features:
- Table metadata JSON
- Snapshot history
- Append-only commits
- Simple schema evolution

Inspired by:
- Apache Iceberg
- Delta Lake
- Hive Metastore

---

## Phase 4 — Query Layer

Learn:
- Apache Arrow memory model
- Vectorized processing
- Query planning
- Predicate pushdown

Features:
- SQL-like query interface
- Partition pruning
- Projection pushdown
- Local execution engine

Possible integrations:
- DataFusion
- Arrow
- Parquet

---

## Phase 5 — Distributed Execution (Optional)

Learn:
- Distributed systems
- Scheduling
- Task coordination
- Fault tolerance

Features:
- Worker nodes
- Distributed scan execution
- Parallel aggregation
- Simple scheduler

---

# Technology Stack

## Language

- Rust (stable)

## Runtime

- Tokio

## Storage

- Local filesystem
- Amazon S3

## Data Formats

- Parquet
- Arrow
- JSON manifests

## Potential Libraries

| Purpose | Library |
|---|---|
| Async runtime | Tokio |
| Serialization | Serde |
| CLI parsing | clap |
| AWS integration | aws-sdk-rust |
| Logging | tracing |
| Columnar memory | Arrow |
| Query engine | DataFusion |
| Error handling | anyhow / thiserror |

---

# Learning Objectives

This project is intentionally designed to exercise core Rust concepts repeatedly:

| Rust Concept | Where It Appears |
|---|---|
| Ownership | File buffers, async tasks |
| Borrowing | Parsers, metadata references |
| Traits | Storage abstraction |
| Lifetimes | Streaming readers |
| Enums | Error handling, query plans |
| Generics | Storage interfaces |
| Async/await | S3 operations |
| Arc/Mutex | Shared runtime state |
| Result/Error propagation | Entire codebase |

---

# Non-Goals

This project is NOT intended to:
- replace production data lake systems
- optimize for maximum performance initially
- support every file format
- build a complete SQL engine immediately

Correctness, clarity, and learning are prioritized over optimization.

---

# Development Environment

## Recommended Setup

- Windows 11
- WSL2 Ubuntu
- RustRover
- Rust toolchain installed in WSL

## Useful Commands

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Test

```bash
cargo test
```

### Format

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

---

# Project Structure (Planned)

```text
src/
  main.rs

  storage/
    local.rs
    s3.rs

  metadata/
    manifest.rs
    snapshot.rs

  table/
    schema.rs
    partition.rs

  query/
    planner.rs
    executor.rs

  formats/
    parquet.rs
```

---

# Long-Term Vision

The long-term goal is to better understand how modern cloud-native analytical systems are designed internally:

- Iceberg
- Delta Lake
- Spark
- DataFusion
- DuckDB
- ClickHouse

By rebuilding simplified versions of these ideas, the project aims to bridge the gap between:
- backend engineering
- infrastructure engineering
- distributed systems
- storage engines
- query processing

---

# Current Status

Current milestone:

```text
[ ] Rust environment setup
[ ] Basic CLI application
[ ] Local filesystem scanner
[ ] Metadata manifest generator
[ ] S3 integration
[ ] Table metadata format
[ ] Query engine
```

---

# Notes

This project is intentionally iterative.

The expectation is:
- rewrite things often
- refactor aggressively
- evolve architecture gradually
- prioritize understanding over perfection

The project itself is part of the learning process.