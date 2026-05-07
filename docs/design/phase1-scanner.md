---
phase: 1
name: Local Metadata Scanner
status: active
last_updated: 2026-05-07
---

# Phase 1 Design — Local Metadata Scanner

## Goal

Scan a local directory tree, detect Parquet and CSV files, infer partition structure from `key=value` path segments, and produce a JSON manifest describing the files found. No async, no network, no external services — just the local filesystem, the type system, and serde.

## Rust concepts this phase exercises

| Concept | Where it appears |
|---|---|
| Structs with derived traits | `FileEntry`, `Manifest` |
| Enums | `FileType`, error enum |
| `#[derive(Debug, Serialize, Deserialize)]` | `FileEntry` and related types |
| Custom error types with `thiserror` | `src/metadata/error.rs` or inline in module |
| `Result<T, E>` and the `?` operator | Every function that touches the filesystem |
| Traits | First sketch of `StorageBackend` |
| `Iterator` combinators | `filter_map`, `collect` in the directory walker |
| `HashMap` | Storing partition key-value pairs |

## Module map

| Module | File | Responsibility |
|---|---|---|
| metadata | `src/metadata/manifest.rs` | Define `FileEntry`, `Manifest`; serialize/deserialize |
| storage | `src/storage/local.rs` | Walk directory, return file metadata |
| (trait) | `src/storage/mod.rs` | First sketch of `StorageBackend` trait |
| binary | `src/main.rs` | Wire together: accept path, run scan, print JSON |

## Key data types (intended shapes)

These are the design intent, not final — the owner writes the actual code and these evolve through review.

```rust
// src/metadata/manifest.rs

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub files: Vec<FileEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub size_bytes: u64,
    pub file_type: FileType,
    pub partitions: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FileType {
    Parquet,
    Csv,
    Unknown,
}
```

```rust
// src/storage/mod.rs — first sketch, will become async in Phase 2

pub trait StorageBackend {
    type Error;
    fn list_files(&self, root: &Path) -> Result<Vec<PathBuf>, Self::Error>;
}
```

## Partition parsing approach

Use `split_once('=')` on each path segment — no regex dependency. A segment like `year=2025` splits into `("year", "2025")`. Segments that don't contain `=` are not partition keys and are skipped. See ADR (decisions.md) if one is added for this.

## Design decisions recorded here

- Partition parsing uses `str::split_once('=')`, not the `regex` crate. Reason: `regex` is an unnecessary dependency for a task that standard string methods handle; `Iterator` combinators are a better learning exercise at this phase.
- `FileEntry` uses owned `String` for `path`, not `&str`. Reason: `FileEntry` values are stored in a `Vec` and serialized, so they must own their data. `&str` would require lifetime annotations that are premature here.
- `Unknown` file types are included in the manifest (not silently skipped). Reason: skipping silently hides information; the manifest should be a faithful record.

## Open questions

- [ ] Should `main.rs` accept a path via `std::env::args()` or hardcode a test path initially? (Decision: start hardcoded, add `std::env::args()` without a crate, defer `clap` to a later phase.)
- [ ] Should the manifest be written to a file or printed to stdout? (Leaning: stdout for Phase 1, file output for Phase 2.)
- [ ] Should `Manifest` include a timestamp or schema version field? (Deferred to Phase 3 when snapshot semantics are designed.)

## What "done" looks like

- `cargo test` passes for partition parsing and `FileEntry` serde round-trip
- `cargo clippy` produces no warnings
- Running `cargo run` against a fixture directory produces valid JSON manifest
- The owner has read and understood every line of code in this phase
- At least one learning milestone in `progress.md` is checked off
