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
