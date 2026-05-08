use std::fs::File;
use std::path::Path;

use arrow_array::RecordBatch;
use arrow_schema::{ArrowError, SchemaRef};
use parquet::arrow::ArrowWriter;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::basic::{Compression, Encoding, ZstdLevel};
use parquet::errors::ParquetError;
use parquet::file::properties::{EnabledStatistics, WriterProperties, WriterVersion};
use thiserror::Error;

// Row groups are the unit of parallel reads and statistics-based pruning.
// These defaults keep row groups large enough for efficient analytical scans
// without allowing one row group to grow without bound.
const DEFAULT_MAX_ROW_GROUP_ROW_COUNT: usize = 1024 * 1024;
const DEFAULT_MAX_ROW_GROUP_BYTES: usize = 128 * 1024 * 1024;
const DEFAULT_DATA_PAGE_SIZE_LIMIT: usize = 1024 * 1024;
const DEFAULT_WRITE_BATCH_SIZE: usize = 1024;

/// Errors returned by Rune's Parquet adapter.
#[derive(Debug, Error)]
pub enum RuneParquetError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parquet error: {0}")]
    Parquet(#[from] ParquetError),

    #[error("Arrow error: {0}")]
    Arrow(#[from] ArrowError),

    #[error("cannot write zero record batches")]
    EmptyInput,

    #[error("record batch at index {batch_index} has a different schema from the first batch")]
    SchemaMismatch { batch_index: usize },
}

/// Write-time Parquet storage choices.
///
/// Readers do not need this configuration because compression, encoding,
/// statistics, and row group metadata are stored inside the Parquet file.
#[derive(Debug, Clone)]
pub struct ParquetWriteOptions {
    /// Compression codec used for all columns.
    pub compression: Compression,

    /// Parquet writer format version.
    pub writer_version: WriterVersion,

    /// Fallback encoding used when dictionary encoding is not applicable.
    pub encoding: Encoding,

    /// Whether dictionary encoding is enabled for all columns.
    pub dictionary_enabled: bool,

    /// Statistics level written for all columns.
    pub statistics_enabled: EnabledStatistics,

    /// Maximum row count per row group, or `None` for unlimited.
    pub max_row_group_row_count: Option<usize>,

    /// Maximum estimated encoded bytes per row group, or `None` for unlimited.
    pub max_row_group_bytes: Option<usize>,

    /// Best-effort target size for each data page.
    pub data_page_size_limit: usize,

    /// Internal column write batch size.
    pub write_batch_size: usize,
}

impl Default for ParquetWriteOptions {
    fn default() -> Self {
        Self {
            // ZSTD is a strong default for modern analytical storage: it usually
            // compresses much better than Snappy while staying broadly supported
            // by current Parquet readers.
            compression: Compression::ZSTD(ZstdLevel::default()),
            writer_version: WriterVersion::PARQUET_2_0,
            encoding: Encoding::PLAIN,
            dictionary_enabled: true,
            statistics_enabled: EnabledStatistics::Page,
            max_row_group_row_count: Some(DEFAULT_MAX_ROW_GROUP_ROW_COUNT),
            max_row_group_bytes: Some(DEFAULT_MAX_ROW_GROUP_BYTES),
            data_page_size_limit: DEFAULT_DATA_PAGE_SIZE_LIMIT,
            write_batch_size: DEFAULT_WRITE_BATCH_SIZE,
        }
    }
}

impl ParquetWriteOptions {
    fn writer_properties(&self) -> WriterProperties {
        WriterProperties::builder()
            .set_writer_version(self.writer_version)
            .set_encoding(self.encoding)
            .set_compression(self.compression)
            .set_dictionary_enabled(self.dictionary_enabled)
            .set_statistics_enabled(self.statistics_enabled)
            .set_max_row_group_row_count(self.max_row_group_row_count)
            .set_max_row_group_bytes(self.max_row_group_bytes)
            .set_data_page_size_limit(self.data_page_size_limit)
            .set_write_batch_size(self.write_batch_size)
            .build()
    }
}

/// Write record batches to a Parquet file using Rune's default write options.
///
/// All batches must have the same schema. The function validates the schemas
/// before creating the output file, so schema mismatches do not leave behind
/// a partially written file.
pub fn write_record_batches(
    path: impl AsRef<Path>,
    batches: &[RecordBatch],
) -> Result<(), RuneParquetError> {
    write_record_batches_with_options(path, batches, &ParquetWriteOptions::default())
}

/// Write record batches to a Parquet file using caller-provided write options.
///
/// This is the entry point to use when the caller wants to customize storage
/// choices such as compression, writer version, encoding, or row group sizing.
pub fn write_record_batches_with_options(
    path: impl AsRef<Path>,
    batches: &[RecordBatch],
    options: &ParquetWriteOptions,
) -> Result<(), RuneParquetError> {
    let Some(first_batch) = batches.first() else {
        return Err(RuneParquetError::EmptyInput);
    };

    let schema = first_batch.schema();
    validate_matching_schemas(&schema, batches)?;

    write_record_batches_with_schema(path.as_ref(), schema, batches, options)
}

fn validate_matching_schemas(
    schema: &SchemaRef,
    batches: &[RecordBatch],
) -> Result<(), RuneParquetError> {
    for (batch_index, batch) in batches.iter().enumerate() {
        if batch.schema().as_ref() != schema.as_ref() {
            return Err(RuneParquetError::SchemaMismatch { batch_index });
        }
    }

    Ok(())
}

fn write_record_batches_with_schema(
    path: &Path,
    schema: SchemaRef,
    batches: &[RecordBatch],
    options: &ParquetWriteOptions,
) -> Result<(), RuneParquetError> {
    let file = File::create(path)?;
    let writer_properties = options.writer_properties();
    let mut writer = ArrowWriter::try_new(file, schema, Some(writer_properties))?;

    for batch in batches {
        writer.write(batch)?;
    }

    writer.close()?;

    Ok(())
}

/// Read all record batches from a Parquet file.
///
/// The reader discovers compression, encoding, schema, and row group metadata
/// from the file itself, so no write options are required.
pub fn read_record_batches(path: impl AsRef<Path>) -> Result<Vec<RecordBatch>, RuneParquetError> {
    let file = File::open(path)?;
    let reader = ParquetRecordBatchReaderBuilder::try_new(file)?.build()?;

    let batches = reader.collect::<Result<Vec<_>, ArrowError>>()?;

    Ok(batches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use arrow_array::{Array, ArrayRef, Int32Array, StringArray};
    use arrow_schema::{DataType, Field, Schema};
    use parquet::schema::types::ColumnPath;

    #[test]
    fn writes_and_reads_record_batches() {
        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Int32, false),
            Field::new("name", DataType::Utf8, false),
        ]));

        let ids: ArrayRef = Arc::new(Int32Array::from(vec![1, 2, 3]));
        let names: ArrayRef = Arc::new(StringArray::from(vec!["Ada", "Grace", "Linus"]));
        let batch = RecordBatch::try_new(schema, vec![ids, names]).expect("valid batch");

        let path = std::env::temp_dir().join("rune_parquet_round_trip.parquet");

        write_record_batches(&path, &[batch]).expect("write should succeed");
        let batches = read_record_batches(&path).expect("read should succeed");

        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].num_rows(), 3);
        assert_eq!(batches[0].num_columns(), 2);

        let id_column = batches[0]
            .column(0)
            .as_any()
            .downcast_ref::<Int32Array>()
            .expect("id column should be int32");
        let name_column = batches[0]
            .column(1)
            .as_any()
            .downcast_ref::<StringArray>()
            .expect("name column should be utf8");

        assert_eq!(id_column.value(0), 1);
        assert_eq!(id_column.value(2), 3);
        assert_eq!(name_column.value(0), "Ada");
        assert_eq!(name_column.value(2), "Linus");
    }

    #[test]
    fn default_write_options_are_explicit() {
        let options = ParquetWriteOptions::default();
        let properties = options.writer_properties();
        let column_path = ColumnPath::from("name");

        assert_eq!(
            properties.compression(&column_path),
            Compression::ZSTD(ZstdLevel::default())
        );
        assert_eq!(properties.writer_version(), WriterVersion::PARQUET_2_0);
        assert_eq!(properties.encoding(&column_path), Some(Encoding::PLAIN));
        assert!(properties.dictionary_enabled(&column_path));
        assert_eq!(
            properties.statistics_enabled(&column_path),
            EnabledStatistics::Page
        );
        assert_eq!(
            properties.max_row_group_row_count(),
            Some(DEFAULT_MAX_ROW_GROUP_ROW_COUNT)
        );
        assert_eq!(
            properties.max_row_group_bytes(),
            Some(DEFAULT_MAX_ROW_GROUP_BYTES)
        );
        assert_eq!(
            properties.data_page_size_limit(),
            DEFAULT_DATA_PAGE_SIZE_LIMIT
        );
        assert_eq!(properties.write_batch_size(), DEFAULT_WRITE_BATCH_SIZE);
    }

    #[test]
    fn writes_with_custom_compression_options() {
        let schema = Arc::new(Schema::new(vec![Field::new("name", DataType::Utf8, false)]));
        let names: ArrayRef = Arc::new(StringArray::from(vec!["Ada", "Grace", "Linus"]));
        let batch = RecordBatch::try_new(schema, vec![names]).expect("valid batch");
        let options = ParquetWriteOptions {
            compression: Compression::SNAPPY,
            ..ParquetWriteOptions::default()
        };

        let path = std::env::temp_dir().join("rune_parquet_snappy.parquet");

        write_record_batches_with_options(&path, &[batch], &options).expect("write should succeed");
        let batches = read_record_batches(&path).expect("read should succeed");

        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].num_rows(), 3);
    }

    #[test]
    fn rejects_empty_batch_list() {
        let path = std::env::temp_dir().join("rune_empty.parquet");
        let error = write_record_batches(path, &[]).expect_err("empty input should fail");

        assert!(matches!(error, RuneParquetError::EmptyInput));
    }

    #[test]
    fn rejects_batches_with_different_schemas() {
        let first_schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int32, false)]));
        let second_schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Int32, false),
            Field::new("name", DataType::Utf8, false),
        ]));

        let first_batch =
            RecordBatch::try_new(first_schema, vec![Arc::new(Int32Array::from(vec![1]))])
                .expect("valid first batch");
        let second_batch = RecordBatch::try_new(
            second_schema,
            vec![
                Arc::new(Int32Array::from(vec![2])),
                Arc::new(StringArray::from(vec!["Ada"])),
            ],
        )
        .expect("valid second batch");

        let path = std::env::temp_dir().join("rune_mismatched_schema.parquet");
        let error = write_record_batches(path, &[first_batch, second_batch])
            .expect_err("schema mismatch should fail");

        assert!(matches!(
            error,
            RuneParquetError::SchemaMismatch { batch_index: 1 }
        ));
    }

    #[test]
    fn does_not_create_file_when_schemas_do_not_match() {
        let first_schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int32, false)]));
        let second_schema = Arc::new(Schema::new(vec![Field::new("name", DataType::Utf8, false)]));

        let first_batch =
            RecordBatch::try_new(first_schema, vec![Arc::new(Int32Array::from(vec![1]))])
                .expect("valid first batch");
        let second_batch = RecordBatch::try_new(
            second_schema,
            vec![Arc::new(StringArray::from(vec!["Ada"]))],
        )
        .expect("valid second batch");

        let path = std::env::temp_dir().join("rune_no_partial_write.parquet");
        let _ = std::fs::remove_file(&path);

        let error = write_record_batches(&path, &[first_batch, second_batch])
            .expect_err("schema mismatch should fail");

        assert!(matches!(
            error,
            RuneParquetError::SchemaMismatch { batch_index: 1 }
        ));
        assert!(!path.exists());
    }
}
