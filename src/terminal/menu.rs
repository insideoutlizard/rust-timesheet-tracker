use crate::entries::{Entry, TimesheetError, load_entries, save_entries};
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
            "{}. [{}] {}: {} — {} min",
            i + 1,
            entry.date.format("%Y-%m-%d %H:%M"),
            entry.project,
            entry.description,
            entry.duration
        );
    }
}

fn add_entry(entries: &mut Vec<Entry>) {
    let mut project = String::new();
    let mut description = String::new();
    let mut duration = String::new();

    print!("Project: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut project).unwrap();

    print!("Description: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut description).unwrap();

    print!("Duration (minutes): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut duration).unwrap();

    let duration: i64 = duration.trim().parse().unwrap_or(0);
    let entry = Entry::new(project.trim(), description.trim(), duration);
    entries.push(entry);

    println!("\nEntry added.");
}

pub fn run_terminal(file_path: &str) -> Result<(), TimesheetError> {
    let mut entries = load_entries(file_path)?;

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
                pause();
            }
            "3" => {
                save_entries(file_path, &entries)?;
                println!("Saved. Exiting...");
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
