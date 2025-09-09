use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Entry {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub project: String,
    pub description: String,
}

impl Entry {
    pub fn new(
        project: &str,
        description: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Self {
        Entry {
            start_time,
            end_time,
            project: project.to_string(),
            description: description.to_string(),
        }
    }

    pub fn duration_minutes(&self) -> i64 {
        (self.end_time - self.start_time).num_minutes()
    }
}
