# Rune — Architecture Decisions

Each entry has an ID, a status, a date, and a record of what was decided and why.
Status is one of: `accepted` | `superseded` (the decision was replaced; old entry stays for history).

When a `[REVERSAL]` entry in `findings.md` maps back to an ADR, note it in the finding: `(see ADR-NNN superseded)`.

---

## ADR-001: `StorageBackend` as the central abstraction
**Date:** 2026-05-07
**Status:** accepted

**Decision:** All code above Phase 1 will depend on a `StorageBackend` trait, not on concrete types (`LocalStorage`, `S3Storage`). The local filesystem implementation in Phase 1 becomes the first concrete type; S3 in Phase 2 becomes the second.

**Why:** This is the primary exercise in Rust traits and generics for this project. Having two concrete implementations forces the trait surface to be designed honestly — a trait with only one implementation is just a renamed struct. It also mirrors how real systems (DataFusion, the `object_store` crate) are designed, so the learning maps to real-world patterns.

**Consequences:** Phase 1 must sketch the trait surface even before S3 exists. The trait will likely need `async_trait` in Phase 2, which means revisiting and possibly changing the Phase 1 trait definition. That revisit is a feature, not a bug — trait evolution is a valuable Rust exercise.

---

## ADR-002: Error handling strategy
**Date:** 2026-05-07
**Status:** accepted

**Decision:** Use `thiserror` for library-style error types in `src/` modules. Use `anyhow` in `main.rs` and integration tests only.

**Why:** `thiserror` forces you to define an explicit error taxonomy — each variant has a name, a message, and an optional source error. This is a better learning exercise than `anyhow`, which accepts anything. The forced taxonomy also produces better error messages and is the idiomatic approach for library code. `anyhow` is reserved for the binary entrypoint where the caller just wants to propagate and print, not match on variants.

**Consequences:** Each module will define its own error enum with `thiserror`. Conversions between error types use `#[from]`, which is where the `From` trait and the `?` operator interact — a good learning exercise in itself.

---

## ADR-003: Flat module file style (no `mod.rs`)
**Date:** 2026-05-07
**Status:** accepted

**Decision:** Use `module_name.rs` for single-file modules. Only use `mod.rs` when a module directory contains sub-modules (i.e., `module_name/mod.rs` plus `module_name/submodule.rs`).

**Why:** The flat `module_name.rs` style is the modern Rust convention (post-2018 edition). It makes directory listings and `grep` results cleaner — you see `storage.rs` instead of multiple files all called `mod.rs`. IDEs also handle it better.

**Consequences:** As modules grow, they start as `src/storage.rs` and are promoted to `src/storage/mod.rs` + sub-files only when there's a genuine need for sub-modules. Don't create the directory structure preemptively.

---

## ADR-004: Use the official Parquet crate
**Date:** 2026-05-07
**Status:** accepted

**Decision:** Rune will use the official Apache Arrow `parquet` crate for Parquet read/write support. The first API boundary is Arrow `RecordBatch`, supported by `arrow-array` and `arrow-schema`.

**Why:** The Parquet file format is complex enough that reimplementing it would obscure the Rust learning goals. Using the official crate keeps the project focused on data lake concepts, typed APIs, error handling, and how Rust code composes existing libraries.

**Consequences:** Arrow concepts arrive earlier than originally planned. The first implementation intentionally stays small: write a slice of `RecordBatch` values to a local file and read a local file back into `RecordBatch` values.

---

## ADR-005: Use Arrow RecordBatch as the in-memory columnar protocol
**Date:** 2026-05-07
**Status:** accepted

**Decision:** Rune will use Arrow `RecordBatch` as the standard in-memory representation for physical columnar data. Format adapters such as `src/formats/parquet.rs` may expose `RecordBatch` directly in their public APIs.

**Why:** Arrow is the standard in-memory columnar representation in the Rust analytical data ecosystem. Parquet is the storage format; Arrow is the memory format. Replacing `RecordBatch` with a Rune-owned wrapper now would mostly add forwarding code without clarifying the learning goals.

**Consequences:** Query and format code may depend on Arrow types. Metadata and table-management code should still use Rune-owned structs such as `Manifest`, `FileEntry`, and future table metadata types. If Rune later needs extra invariants or a narrower API, it can introduce a wrapper around `RecordBatch` at that point.
