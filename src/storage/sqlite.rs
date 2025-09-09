use crate::entries::Entry;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, params};

pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection.open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            project TEXT NOT NULL,
            description TEXT,
            duration INTEGER GENERATED ALWAYS AS (
                CAST((julianday(end_time) - julianday(start_time)) * 1440 AS INTEGER)
            ) STORED
            ",
        [],
    )?;
    Ok(conn)
}

pub fn insert_entry(conn: &Connection, entry: &Entry) -> Result<()> {
    conn.execute(
        "INSERT INTO entries (date, project, description, duration) VALUES (?1. ?2, ?3, ?4)",
        params![
            entry.date.to_rfc3339(),
            entry.project,
            entry.description,
            entry.duration
        ],
    )?;
    Ok(())
}

pub fn get_all_entries(conn: &Connection) -> Result<Vec<Entry>> {
    let mut stmt = conn.prepare("SELECT date, project, description, duration FROM entries")?;
    let rows = stmt.query_map([], |row| {
        let date_str: String = row.get(0)?;
        let date = DateTime::parse_from_rfc3339(&date_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()); // fallback if parsing fails

        Ok(Entry {
            date,
            project: row.get(1)?,
            description: row.get(2)?,
            duration: row.get(3)?,
        })
    })?;

    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    Ok(entries)
}
