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
    /// Creates a new database instance, loading existing data from the specified path if present.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the database file.
    /// * `format` - Optional. The format of the storage, defaults to `StorageFormat::Binary` if `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use solokv::{Database, StorageFormat};
    /// use tempfile::NamedTempFile;
    ///
    /// let temp_file = NamedTempFile::new().unwrap();
    /// let db_path = temp_file.path().to_str().unwrap();
    ///
    /// let db = Database::<String, i32>::new(db_path, Some(StorageFormat::Json)).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an issue reading from the specified path or if deserialization fails.
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

    /// Returns a vector of references to all keys in the database.
    ///
    /// # Examples
    ///
    /// ```
    /// # use solokv::{Database, StorageFormat};
    /// # use tempfile::NamedTempFile;
    /// # let temp_file = NamedTempFile::new().unwrap();
    /// # let db_path = temp_file.path().to_str().unwrap();
    /// # let db = Database::<String, i32>::new(db_path, None).unwrap();
    /// let keys = db.keys();
    /// ```
    pub fn keys(&self) -> Vec<&K> {
        self.data.keys().collect()
    }

    /// Retrieves the value associated with a given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the key being retrieved.
    ///
    /// # Examples
    ///
    /// ```
    /// # use solokv::{Database, StorageFormat};
    /// # use tempfile::NamedTempFile;
    /// # let temp_file = NamedTempFile::new().unwrap();
    /// # let db_path = temp_file.path().to_str().unwrap();
    /// # let mut db = Database::<String, String>::new(db_path, None).unwrap();
    /// db.put("Hello".to_string(), Some("World".to_string())).unwrap();
    /// assert_eq!(db.get(&"Hello".to_string()).unwrap(), &"World".to_string());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `KeyNotFoundError` if the key is not present in the database.
    pub fn get(&self, key: &K) -> Result<&V, DatabaseError> {
        self.data
            .get(key)
            .ok_or_else(|| DatabaseError::KeyNotFoundError(format!("{:?}", key)))
    }

    /// Inserts or updates a key-value pair in the database. If `value` is `None`, the key is removed.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert or update.
    /// * `value` - The value to associate with the key; if `None`, the key is removed from the database.
    ///
    /// # Examples
    ///
    /// ```
    /// # use solokv::{Database, StorageFormat};
    /// # use tempfile::NamedTempFile;
    /// # let temp_file = NamedTempFile::new().unwrap();
    /// # let db_path = temp_file.path().to_str().unwrap();
    /// # let mut db = Database::<String, String>::new(db_path, None).unwrap();
    /// db.put("key1".to_string(), Some("value1".to_string())).unwrap();
    /// assert_eq!(db.get(&"key1".to_string()).unwrap(), &"value1".to_string());
    ///
    /// db.put("key1".to_string(), None).unwrap(); // This removes "key1" from the database
    /// assert!(db.get(&"key1".to_string()).is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an issue saving the data to the path.
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

    /// Checks if a key exists in the database.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the key being checked.
    ///
    /// # Examples
    ///
    /// ```
    /// # use solokv::{Database, StorageFormat};
    /// # use tempfile::NamedTempFile;
    /// # let temp_file = NamedTempFile::new().unwrap();
    /// # let db_path = temp_file.path().to_str().unwrap();
    /// # let mut db = Database::<String, String>::new(db_path, None).unwrap();
    /// db.put("some_key".to_string(), Some("some_value".to_string())).unwrap();
    /// assert!(db.exists(&"some_key".to_string()));
    /// ```
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
