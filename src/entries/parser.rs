use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub date: DateTime<Utc>,
    pub project: String,
    pub description: String,
    pub duration: i64, // minutes
}

impl Entry {
    pub fn new(project: &str, description: &str, duration: i64) -> Self {
        Entry {
            date: Utc::now(),
            project: project.to_string(),
            description: description.to_string(),
            duration,
        }
    }
}
