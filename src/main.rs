use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    date: DateTime<Utc>,
    project: String,
    description: String,
    duration: i64,
}

#[derive(Debug, Error)]
enum TimesheetError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

fn load_entries(file_path: &str) -> Result<Vec<Entry>, TimesheetError> {
    match fs::read_to_string(file_path) {
        Ok(data) => {
            let entries: Vec<Entry> = serde_json::from_str(&data)?;
            Ok(entries)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            Ok(Vec::new()) // no file yet? start fresh
        }
        Err(e) => Err(TimesheetError::Io(e)),
    }
}

fn main() {
    let file_path = "timesheet.json";

    match load_entries(file_path) {
        Ok(entries) => {
            println!("Loaded {} entries:", entries.len());
            for entry in entries {
                println!(
                    "{} | {} | {} | {} minutes",
                    entry.date, entry.project, entry.description, entry.duration
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to load entries: {}", e);
        }
    }
}
