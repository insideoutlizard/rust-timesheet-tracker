use crate::entries::parser::Entry;
use serde_json;
use std::fs;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimesheetError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn load_entries(file_path: &str) -> Result<Vec<Entry>, TimesheetError> {
    match fs::read_to_string(file_path) {
        Ok(data) => {
            let entries: Vec<Entry> = serde_json::from_str(&data)?;
            Ok(entries)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(TimesheetError::Io(e)),
    }
}

pub fn save_entries(file_path: &str, entries: &Vec<Entry>) -> Result<(), TimesheetError> {
    let json = serde_json::to_string_pretty(entries)?;
    fs::write(file_path, json)?;
    Ok(())
}
