# Rune — Findings

This file records things that must not be forgotten or re-debated between sessions.
Scan the active phase's section at the start of every session.

**Categories:**
- `[RUST]` — A Rust language mistake or surprise, with the correction and the one-sentence principle behind it.
- `[REVERSAL]` — A design decision that was made and then reversed. Records both the original and the current approach, and why the change was made.
- `[RESOLVED]` — A question or debate that has been fully settled. Do not reopen.

---

## Phase 1 — Local Metadata Scanner

*(No entries yet — add the first one when the first mistake or resolved debate occurs.)*

---

## Phase 2 — S3-backed Data Lake

*(empty — phase not started)*

---

## Phase 3 — Table Format

*(empty — phase not started)*

---

## Phase 4 — Query Layer

*(empty — phase not started)*

---

## Cross-phase findings

*(Rust language findings that are not specific to one phase go here.)*

- [RUST] Prefer `impl Trait` for simple one-off parameter bounds, such as `path: impl AsRef<Path>`. Use a named generic with a `where` clause when the type parameter appears in multiple places to enforce “same concrete type”, or when naming the type makes the signature easier to understand. Principle: introduce a generic type name only when the name carries information or expresses a real type relationship.
- [RUST] Prefer borrowed slices like `&[RecordBatch]` when a function only needs to read caller-owned data, and return owned containers like `Vec<RecordBatch>` when the function creates data that must outlive the function call. Principle: borrow inputs you do not need to own; return ownership for values created inside the function.
- [RUST] A Rust function returns one error type in `Result<T, E>`. For library code that can fail in multiple ways, define an enum that aggregates the possible error cases, then use `#[from]` conversions so `?` can wrap underlying errors into that enum. Principle: Rust error handling is explicit type conversion into one declared error type, not exception hierarchy matching.
