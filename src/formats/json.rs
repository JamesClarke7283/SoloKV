use super::super::DatabaseError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::{Read, Write};

pub(crate) fn serialize<K, V, W>(writer: &mut W, data: &HashMap<K, V>) -> Result<(), DatabaseError>
where
    K: Serialize + Debug,
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
