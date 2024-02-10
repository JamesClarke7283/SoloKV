mod formats;
mod util;

use crate::util::file;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use thiserror::Error;

#[cfg(feature = "logging")]
use std::sync::Once;

#[cfg(feature = "logging")]
static LOGGER_INIT: Once = Once::new();

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StorageFormat {
    Binary,
    Json,
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
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
    K: std::hash::Hash + std::cmp::Eq + for<'de> Deserialize<'de> + Serialize + Debug,
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
    /// use libsolokv::{Database, StorageFormat};
    /// use tempfile::NamedTempFile;
    ///
    /// let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    /// let db_path = temp_file.path().to_str().expect("Failed to get temp file path");
    ///
    /// let db = Database::<String, i32>::new(db_path, Some(StorageFormat::Json))
    ///     .expect("Failed to create database");
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an issue reading from the specified path or if deserialization fails.
    /// Possible errors include `IoError` for I/O issues, `SerdeError` for serialization/deserialization issues,
    /// `PermissionError` for insufficient access, `InvalidFormatError` for specifying an invalid storage format,
    /// and `MalformedFileError` for detecting corrupted data.
    pub fn new(path: &str, format: Option<StorageFormat>) -> Result<Self, DatabaseError> {
        #[cfg(feature = "logging")]
        LOGGER_INIT.call_once(|| {
            env_logger::init();
        });
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
    /// # use libsolokv::{Database, StorageFormat};
    /// # use tempfile::NamedTempFile;
    /// # let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    /// # let db_path = temp_file.path().to_str().expect("Failed to get temp file path");
    /// # let db = Database::<String, i32>::new(db_path, None).expect("Failed to create database");
    /// let keys = db.keys();
    /// ```
    ///
    /// # Errors
    ///
    /// This function can return `IoError` if there's an issue accessing the database file.
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
    /// # use libsolokv::{Database, StorageFormat};
    /// # use tempfile::NamedTempFile;
    /// let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    /// let db_path = temp_file.path().to_str().expect("Failed to get temp file path");
    /// let db = Database::<String, String>::new(db_path, None);
    /// match db {
    ///     Ok(mut db) => {
    ///         db.put("Hello".to_string(), Some("World".to_string())).expect("Failed to insert value");
    ///         match db.get(&"Hello".to_string()) {
    ///             Ok(value) => assert_eq!(value, &"World".to_string()),
    ///             Err(e) => panic!("Failed to retrieve value: {:?}", e),
    ///         }
    ///     },
    ///     Err(e) => panic!("Failed to create database: {:?}", e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `KeyNotFoundError` if the key is not present in the database. Other possible errors include `IoError` and `SerdeError`.
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
    /// # use libsolokv::{Database, StorageFormat};
    /// # use tempfile::NamedTempFile;
    /// # let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    /// # let db_path = temp_file.path().to_str().expect("Failed to get temp file path");
    /// # let mut db = Database::<String, String>::new(db_path, None).expect("Failed to create database");
    /// db.put("key1".to_string(), Some("value1".to_string())).expect("Failed to insert value");
    /// assert_eq!(db.get(&"key1".to_string()).expect("Failed to retrieve value"), &"value1".to_string());
    ///
    /// db.put("key1".to_string(), None).expect("Failed to remove key");
    /// assert!(db.get(&"key1".to_string()).is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an issue saving the data to the path. Possible errors include `IoError`, `SerdeError`, and `PermissionError`.
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
    /// # use libsolokv::{Database, StorageFormat};
    /// # use tempfile::NamedTempFile;
    /// # let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    /// # let db_path = temp_file.path().to_str().expect("Failed to get temp file path");
    /// # let mut db = Database::<String, String>::new(db_path, None).expect("Failed to create database");
    /// db.put("some_key".to_string(), Some("some_value".to_string())).expect("Failed to insert value");
    /// assert!(db.exists(&"some_key".to_string()));
    /// ```
    ///
    /// # Errors
    ///
    /// Can return `IoError` if there's an issue accessing the database file. Checks for existence do not typically result in `KeyNotFoundError`.
    pub fn exists(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    // Helper function to create a new database instance with a temporary file
    fn new_temp_db(format: StorageFormat) -> (Database<String, String>, NamedTempFile) {
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        let db_path = temp_file.path().to_str().unwrap();
        // Ensure the directory exists
        if let Some(parent) = PathBuf::from(db_path).parent() {
            fs::create_dir_all(parent).expect("Failed to create directory for database file");
        }
        let db: Database<String, String> =
            Database::new(db_path, Some(format)).expect("Failed to create new database");
        (db, temp_file)
    }

    #[test]
    fn test_database_new() {
        let (mut db, temp_file) = new_temp_db(StorageFormat::Json);
        // Ensure some data is put in the database to trigger file creation.
        db.put("init_key".to_string(), Some("init_value".to_string()))
            .unwrap();
        // Explicitly check the file exists after operation
        assert!(
            temp_file.path().exists(),
            "Database file should exist after creation"
        );
    }

    #[test]
    fn test_database_put_get() {
        let (mut db, temp_file) = new_temp_db(StorageFormat::Json);
        db.put("key1".to_string(), Some("value1".to_string()))
            .unwrap();
        assert_eq!(db.get(&"key1".to_string()).unwrap(), &"value1".to_string());
        assert!(
            temp_file.path().exists(),
            "Database file should exist after put operation"
        );
    }

    #[test]
    fn test_database_keys() {
        let (mut db, temp_file) = new_temp_db(StorageFormat::Json);
        db.put("key1".to_string(), Some("value1".to_string()))
            .unwrap();
        db.put("key2".to_string(), Some("value2".to_string()))
            .unwrap();
        let keys = db.keys();
        assert!(keys.contains(&&"key1".to_string()));
        assert!(keys.contains(&&"key2".to_string()));
        assert!(
            temp_file.path().exists(),
            "Database file should exist after adding keys"
        );
    }

    #[test]
    fn test_database_exists() {
        let (mut db, temp_file) = new_temp_db(StorageFormat::Json);
        db.put("key1".to_string(), Some("value1".to_string()))
            .unwrap();
        assert!(db.exists(&"key1".to_string()));
        assert!(
            temp_file.path().exists(),
            "Database file should exist after checking existence"
        );
    }

    #[test]
    fn test_database_delete() {
        let (mut db, temp_file) = new_temp_db(StorageFormat::Json);
        db.put("key1".to_string(), Some("value1".to_string()))
            .unwrap();
        db.put("key1".to_string(), None).unwrap();
        assert!(!db.exists(&"key1".to_string()));
        assert!(
            temp_file.path().exists(),
            "Database file should exist after delete operation"
        );
    }
}
