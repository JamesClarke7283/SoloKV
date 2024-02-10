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
    K: Serialize + Debug,
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
    use std::fs;
    use tempfile::tempdir;

    // Example refactored test function using tempfile more effectively
    #[test]
    fn test_save_json_format() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("solokv_test_save_json.json");
        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("key1".to_string(), "value1".to_string());
        data.insert("key2".to_string(), "value2".to_string());

        // Attempt to save the data
        assert!(
            save(&file_path, StorageFormat::Json, &data).is_ok(),
            "Failed to save data"
        );

        // Verify file contents
        let contents = fs::read_to_string(file_path)
            .expect("File should exist and be readable after save operation");
        assert!(
            contents.contains("\"key1\":\"value1\"") && contents.contains("\"key2\":\"value2\"")
        );

        // Temporary directory and file are automatically cleaned up when `dir` goes out of scope
    }

    #[test]
    fn test_load_json_format() {
        let dir = tempdir().unwrap(); // Create a temporary directory
        let file_path = dir.path().join("test_load_json_format.json"); // Temporary file path
        let mut expected_data: HashMap<String, String> = HashMap::new();
        expected_data.insert("key1".to_string(), "value1".to_string());
        expected_data.insert("key2".to_string(), "value2".to_string());

        // Write test data to the temporary file
        let json_content = serde_json::to_string(&expected_data).unwrap();
        fs::write(&file_path, json_content).expect("Failed to write test data to file");

        // Attempt to load the data
        let mut loaded_data: HashMap<String, String> = HashMap::new();
        assert!(load(&file_path, StorageFormat::Json, &mut loaded_data).is_ok());
        assert_eq!(
            loaded_data, expected_data,
            "Loaded data does not match expected data"
        );

        // Temporary directory and file are automatically cleaned up
    }

    // Ensure other test functions are correctly implemented as well
}
