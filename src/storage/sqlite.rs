use crate::entries::Entry;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, params};
use std::path::Path;

fn db_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn create_db(conn: &Connection) -> Result<()> {
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
            )",
        [],
    )?;

    println!("Database schema created.");
    Ok(())
}

pub fn init_db(path: &str) -> Result<Connection> {
    if !db_exists(path) {
        println!("Database not found. Creating...");
        let conn = Connection::open(path)?;
        create_db(&conn)?;
        Ok(conn)
    } else {
        println!("Database found.");
        Connection::open(path)
    }
}

pub fn insert_entry(conn: &Connection, entry: &Entry) -> Result<()> {
    conn.execute(
        "INSERT INTO entries (start_time, end_time, project, description) VALUES (?1, ?2, ?3, ?4)",
        params![
            entry.start_time.to_rfc3339(),
            entry.end_time.to_rfc3339(),
            entry.project,
            entry.description
        ],
    )?;
    Ok(())
}

pub fn get_all_entries(conn: &Connection) -> Result<Vec<Entry>> {
    let mut stmt = conn.prepare(
        "SELECT start_time, end_time, project, description FROM entries ORDER BY start_time ASC",
    )?;
    let rows = stmt.query_map([], |row| {
        let start_str: String = row.get(0)?;
        let end_str: String = row.get(1)?;
        let start_time = DateTime::parse_from_rfc3339(&start_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(Utc::now());
        let end_time = DateTime::parse_from_rfc3339(&end_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(Utc::now());

        Ok(Entry::new(
            &row.get::<_, String>(2)?,
            &row.get::<_, String>(3)?,
            start_time,
            end_time,
        ))
    })?;

    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    Ok(entries)
}
