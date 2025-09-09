mod entries;

use entries::{Entry, TimesheetError, load_entries, save_entries};

fn main() -> Result<(), TimesheetError> {
    let file_path = "timesheet.json";

    let mut entries = load_entries(file_path)?;

    let new_entry = Entry::new(
        "Daily activities",
        "shit down everyones throat.",
        9999999999,
    );

    entries.push(new_entry);
    save_entries(file_path, &entries)?;

    println!("Entry was saved.");
    Ok(())
}
