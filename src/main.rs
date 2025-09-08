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
    duration: i64, // minutes
}

#[derive(Debug, Error)]
enum TimesheetError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

// read json file from disk, deserialize JSON into Vec<Entry>
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

// serialize updated timesheet, write to disk
fn save_entries(file_path: &str, entries: &Vec<Entry>) -> Result<(), TimesheetError> {
    let json = serde_json::to_string_pretty(entries)?;
    fs::write(file_path, json)?;
    Ok(())
}

// modify existing Vec<Entry>, requires mutable reference
fn add_entry(entries: &mut Vec<Entry>, project: &str, description: &str, duration: i64) {
    let new_entry = Entry {
        date: Utc::now(),
        project: project.to_string(),
        description: description.to_string(),
        duration,
    };
    entries.push(new_entry);
}

fn main() -> Result<(), TimesheetError> {
    let file_path = "timesheet.json";

    //load existing entries
    let mut entries = load_entries(file_path)?;

    // add test entry
    add_entry(&mut entries, "General Project", "Initail setup", 23);

    // save entry to list and write to file
    save_entries(file_path, &entries)?;

    println!("Entry was saved.");
    Ok(())
}

// fn main() {
//     let file_path = "timesheet.json";

//     match load_entries(file_path) {
//         Ok(entries) => {
//             println!("Loaded {} entries:", entries.len());
//             for entry in entries {
//                 println!(
//                     "{} | {} | {} | {} minutes",
//                     entry.date, entry.project, entry.description, entry.duration
//                 );
//             }
//         }
//         Err(e) => {
//             eprintln!("Failed to load entries: {}", e);
//         }
//     }
// }
