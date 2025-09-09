use crate::entries::Entry;
use crate::storage::sqlite::{get_all_entries, init_db, insert_entry};
use chrono::{Local, NaiveDateTime, TimeZone, Utc};
use std::io::{self, Write};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn pause() {
    println!("\nPress Enter to continue...");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn show_menu() {
    println!("╔════════════════════════════╗");
    println!("║   ACTIVITY TRACKER MENU    ║");
    println!("╠════════════════════════════╣");
    println!("║ 1. View Entries            ║");
    println!("║ 2. Add Entry               ║");
    println!("║ 3. Save & Exit             ║");
    println!("╚════════════════════════════╝");
    print!("> ");
    io::stdout().flush().unwrap();
}

fn view_entries(entries: &Vec<Entry>) {
    println!("\n─── Entries ───");
    for (i, entry) in entries.iter().enumerate() {
        println!(
            "{}. [{} -> {}] {}: {} — {} min",
            i + 1,
            entry.start_time.format("%Y-%m-%d %H:%M"),
            entry.end_time.format("%Y-%m-%d %H:%M"),
            entry.project,
            entry.description,
            entry.duration_minutes()
        );
    }
}

fn add_entry(entries: &mut Vec<Entry>) {
    let mut project = String::new();
    let mut description = String::new();
    let mut start_input = String::new();
    let mut end_input = String::new();

    print!("Project: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut project).unwrap();

    print!("Description: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut description).unwrap();

    // manual input, leaving blank should get current time
    print!("Start time: (YYYY-MM-DD HH:MM): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut start_input).unwrap();

    print!("Use current time as start? (y/nm): ");
    io::stdout().flush().unwrap();
    let mut use_now = String::new();
    io::stdin().read_line(&mut use_now).unwrap();

    let start_time = if use_now.trim().eq_ignore_ascii_case("y") {
        Utc::now()
    } else {
        print!("Start time: (YYYY-MM-DD HH:MM): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut start_input).unwrap();
        let naive = NaiveDateTime::parse_from_str(start_input.trim(), "%Y-%m-%d %H:%M").unwrap();
        Local
            .from_local_datetime(&naive)
            .unwrap()
            .with_timezone(&Utc)
    };

    // manual input, leaving blank should add the entry to a list of unfinished entries
    print!("End time: (YYYY-MM-DD HH:MM): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut end_input).unwrap();
    let naive = NaiveDateTime::parse_from_str(end_input.trim(), "%Y-%m-%d %H:%M").unwrap();
    let end_time = Local
        .from_local_datetime(&naive)
        .unwrap()
        .with_timezone(&Utc);

    let entry = Entry::new(project.trim(), description.trim(), start_time, end_time);
    entries.push(entry);

    println!("\nEntry added.");
}

pub fn run_terminal(db_path: &str) -> rusqlite::Result<()> {
    let conn = init_db(db_path)?;
    let mut entries = get_all_entries(&conn)?;

    loop {
        clear_screen();
        show_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clear_screen();
                view_entries(&entries);
                pause();
            }
            "2" => {
                clear_screen();
                add_entry(&mut entries);
                let new_entry = entries.last().unwrap();
                insert_entry(&conn, new_entry)?;
                pause();
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option.");
                pause();
            }
        }
    }

    Ok(())
}
