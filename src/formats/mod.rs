//! Format adapters translate between Rune's in-memory data representation
//! and physical file formats such as Parquet.

/// Parquet read/write support backed by the official Apache Arrow `parquet` crate.
pub mod parquet;
