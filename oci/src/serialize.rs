use serde;
use serde_json;

use std::fs::File;
use std::io;

use crate::errors::*;

pub fn to_writer<W: io::Write, T: serde::Serialize>(
    obj: &T,
    mut writer: W,
) -> Result<()> {
    Ok(serde_json::to_writer(&mut writer, &obj)?)
}

// pub fn from_reader<'de, R: io::Read, T: serde::Deserialize<'de>>(reader: R)
//                                           -> Result<T> {
//     Ok(serde_json::from_reader(reader)?)
// }

pub fn serialize<T: serde::Serialize>(obj: &T, path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    Ok(serde_json::to_writer(&mut file, &obj)?)
}

pub fn deserialize<'de, T: serde::de::DeserializeOwned>(
    path: &str,
) -> Result<T> {
    let file = File::open(path)?;
    Ok(serde_json::from_reader(&file)?)
}

pub fn to_string<T: serde::Serialize>(obj: &T) -> Result<String> {
    Ok(serde_json::to_string(&obj)?)
}
