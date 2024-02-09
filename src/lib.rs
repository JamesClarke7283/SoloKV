mod formats;
mod util;

use crate::util::file;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StorageFormat {
    Binary,
    Json,
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error")]
    SerdeError,
    #[error("Key not found: {0:?}")]
    KeyNotFoundError(String),
    #[error("Permission error: insufficient access rights to {0}")]
    PermissionError(String),
    #[error("Invalid format error: the specified format {0} is incorrect.")]
    InvalidFormatError(String),
    #[error("Malformed file error: data in {0} is corrupted or malformed.")]
    MalformedFileError(String),
}

#[derive(Debug)]
pub struct Database<K, V> {
    data: HashMap<K, V>,
    path: PathBuf,
    format: StorageFormat,
}

impl<K, V> Database<K, V>
where
    K: std::hash::Hash + std::cmp::Eq + std::fmt::Debug + for<'de> Deserialize<'de> + Serialize,
    V: std::fmt::Debug + for<'de> Deserialize<'de> + Serialize,
{
    pub fn new(path: &str, format: Option<StorageFormat>) -> Result<Self, DatabaseError> {
        let path = PathBuf::from(path);
        let format = format.unwrap_or(StorageFormat::Binary);
        let mut database = Self {
            data: HashMap::new(),
            path,
            format,
        };

        file::load(&database.path, database.format, &mut database.data)?;

        Ok(database)
    }

    #[must_use]
    pub fn keys(&self) -> Vec<&K> {
        self.data.keys().collect()
    }

    pub fn get(&self, key: &K) -> Result<&V, DatabaseError> {
        self.data
            .get(key)
            .ok_or_else(|| DatabaseError::KeyNotFoundError(format!("{:?}", key)))
    }

    pub fn put(&mut self, key: K, value: Option<V>) -> Result<(), DatabaseError> {
        match value {
            Some(val) => {
                self.data.insert(key, val);
            }
            None => {
                self.data.remove(&key);
            }
        }

        file::save(&self.path, self.format, &self.data)?;

        Ok(())
    }

    pub fn exists(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_db_path(format: &str) -> String {
        format!("/tmp/solokv_test_db_{}.json", format)
    }

    #[test]
    fn test_database_new() {
        let path = temp_db_path("new");
        {
            let mut db: Database<String, String> = Database::new(&path, Some(StorageFormat::Json))
                .expect("Failed to create new database");
            // Ensure some data is put in the database to trigger file creation.
            db.put("init_key".to_string(), Some("init_value".to_string()))
                .unwrap();
        }
        assert!(
            PathBuf::from(&path).exists(),
            "Database file was not created."
        );
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_database_put_get() {
        let path = temp_db_path("put_get");
        let mut db: Database<String, String> = Database::new(&path, Some(StorageFormat::Json))
            .expect("Failed to create database for put/get test");

        db.put("key1".to_string(), Some("value1".to_string()))
            .unwrap();
        assert_eq!(db.get(&"key1".to_string()).unwrap(), &"value1".to_string());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_database_keys() {
        let path = temp_db_path("keys");
        let mut db: Database<String, String> = Database::new(&path, Some(StorageFormat::Json))
            .expect("Failed to create database for keys test");

        db.put("key1".to_string(), Some("value1".to_string()))
            .unwrap();
        db.put("key2".to_string(), Some("value2".to_string()))
            .unwrap();

        let keys = db.keys();
        assert!(keys.contains(&&"key1".to_string()));
        assert!(keys.contains(&&"key2".to_string()));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_database_exists() {
        let path = temp_db_path("exists");
        let mut db: Database<String, String> = Database::new(&path, Some(StorageFormat::Json))
            .expect("Failed to create database for exists test");

        db.put("key1".to_string(), Some("value1".to_string()))
            .unwrap();
        assert!(db.exists(&"key1".to_string()));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_database_delete() {
        let path = temp_db_path("delete");
        let mut db: Database<String, String> = Database::new(&path, Some(StorageFormat::Json))
            .expect("Failed to create database for delete test");

        db.put("key1".to_string(), Some("value1".to_string()))
            .unwrap();
        db.put("key1".to_string(), None).unwrap();
        assert!(!db.exists(&"key1".to_string()));

        fs::remove_file(path).unwrap();
    }
}
