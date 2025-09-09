// this file is to so that main.rs doesn't end up looking terrible
// pub mod loader;
pub mod parser;

// pub use loader::{TimesheetError, load_entries, save_entries};
pub use parser::Entry;
