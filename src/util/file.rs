use crate::formats::{binary, json};
use crate::{DatabaseError, StorageFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

pub fn save<K, V>(
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
        StorageFormat::Binary => binary::serialize(&mut buf_writer, data)?, // Correct usage of mutable reference.
        StorageFormat::Json => json::serialize(&mut buf_writer, data)?, // Correct usage of mutable reference.
    }

    Ok(())
}

pub fn load<K, V>(
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
