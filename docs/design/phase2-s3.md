---
phase: 2
name: S3-backed Data Lake
status: draft
last_updated: 2026-05-07
---

# Phase 2 Design — S3-backed Data Lake

*(Stub — this doc will be designed in detail when Phase 1 is complete.)*

## Anticipated Rust concepts

- `async fn` and `.await`
- `tokio` runtime and `#[tokio::main]`
- `Arc<T>` for shared ownership across async tasks
- `async_trait` for `StorageBackend` (the trait defined in Phase 1 becomes async here)
- Ownership across `.await` points — a common source of borrow checker surprises

## Key unknowns to resolve at design time

- How does `StorageBackend` change when methods become `async`? (The Phase 1 synchronous trait will likely need to be replaced or extended.)
- Should S3 credentials come from environment variables, a config file, or the AWS default credential chain?
- What is the right level of parallelism for directory listing — `tokio::spawn` per file, per prefix, or something else?
- How does streaming IO (`AsyncRead`/`AsyncWrite`) differ from the synchronous file IO used in Phase 1?
