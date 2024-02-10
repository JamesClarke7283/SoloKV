use super::super::DatabaseError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::{Read, Write};

pub(crate) fn serialize<K, V, W>(writer: &mut W, data: &HashMap<K, V>) -> Result<(), DatabaseError>
where
    K: Serialize,
    V: Serialize,
    W: Write,
{
    serde_json::to_writer(writer, data).map_err(|_| DatabaseError::SerdeError)
}

pub(crate) fn deserialize<K, V, R>(reader: R, data: &mut HashMap<K, V>) -> Result<(), DatabaseError>
where
    R: Read,
    K: for<'de> Deserialize<'de> + Serialize + Eq + Hash + Debug, // Adjusted
    V: for<'de> Deserialize<'de> + Serialize,                     // Adjusted
{
    *data = serde_json::from_reader(reader).map_err(|_| DatabaseError::SerdeError)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::io::Read;
    use tempfile::NamedTempFile;

    #[test]
    fn test_serialize() {
        let mut data = HashMap::new();
        data.insert("key1", "value1");
        data.insert("key2", "value2");

        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        serialize(&mut temp_file, &data).expect("Failed to serialize data");

        let mut contents = String::new();
        temp_file
            .reopen()
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert!(contents.contains("\"key1\":\"value1\""));
        assert!(contents.contains("\"key2\":\"value2\""));
    }

    #[test]
    fn test_deserialize() {
        let json_content = r#"{"key1":"value1","key2":"value2"}"#;
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        fs::write(temp_file.path(), json_content).expect("Failed to write to temporary file");

        let mut data: HashMap<String, String> = HashMap::new();
        let file = fs::File::open(temp_file.path()).unwrap();
        deserialize(file, &mut data).expect("Failed to deserialize data");

        assert_eq!(data.get("key1"), Some(&"value1".to_string()));
        assert_eq!(data.get("key2"), Some(&"value2".to_string()));
    }
}
