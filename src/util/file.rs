use crate::formats::{binary, json};
use crate::{DatabaseError, StorageFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

pub(crate) fn save<K, V>(
    path: &PathBuf,
    format: StorageFormat,
    data: &HashMap<K, V>,
) -> Result<(), DatabaseError>
where
    K: Serialize + Debug, // Ensure Debug is included if required by the serialization functions.
    V: Serialize,
{
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let mut buf_writer = BufWriter::new(file); // Correctly declare as mutable.

    match format {
        StorageFormat::Binary => binary::serialize(&mut buf_writer, data)?,
        StorageFormat::Json => json::serialize(&mut buf_writer, data)?,
    }

    Ok(())
}

pub(crate) fn load<K, V>(
    path: &PathBuf,
    format: StorageFormat,
    data: &mut HashMap<K, V>,
) -> Result<(), DatabaseError>
where
    K: for<'de> Deserialize<'de> + Serialize + std::hash::Hash + std::cmp::Eq + Debug, // Ensure Debug is included if required.
    V: for<'de> Deserialize<'de> + Serialize,
{
    if path.exists() {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);

        match format {
            StorageFormat::Binary => binary::deserialize(buf_reader, data)?,
            StorageFormat::Json => json::deserialize(buf_reader, data)?,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::{Read, Write};
    use tempfile::tempdir;

    fn setup_test_environment() -> (PathBuf, HashMap<String, String>) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_db.json");
        let mut data = HashMap::new();
        data.insert("key1".to_string(), "value1".to_string());
        data.insert("key2".to_string(), "value2".to_string());
        (path, data)
    }

    #[test]
    fn test_save_json_format() {
        let (path, data) = setup_test_environment();
        let save_result = save(&path, StorageFormat::Json, &data);
        assert!(
            save_result.is_ok(),
            "Failed to save data: {:?}",
            save_result
        );

        let mut file = File::open(&path).expect("File should exist after save operation");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Should be able to read the file contents");
        assert!(
            contents.contains("\"key1\":\"value1\"") && contents.contains("\"key2\":\"value2\"")
        );

        // Clean up
        fs::remove_file(path).expect("Failed to clean up test file");
    }

    #[test]
    fn test_load_json_format() {
        let (path, expected_data) = setup_test_environment();
        // Prepopulate the file with JSON data
        let json_content = serde_json::to_string(&expected_data).unwrap();
        let mut file = File::create(&path).expect("Failed to create test file for load operation");
        writeln!(file, "{}", json_content).expect("Failed to write test data to file");

        let mut loaded_data: HashMap<String, String> = HashMap::new();
        let load_result = load(&path, StorageFormat::Json, &mut loaded_data);
        assert!(
            load_result.is_ok(),
            "Failed to load data: {:?}",
            load_result
        );
        assert_eq!(
            loaded_data, expected_data,
            "Loaded data does not match expected data"
        );

        // Clean up
        fs::remove_file(path).expect("Failed to clean up test file");
    }

    // TODO: Additional tests for binary format
}
