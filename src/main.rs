use crate::terminal::run_terminal;

mod entries;
mod storage;
mod terminal;

fn main() {
    if let Err(e) = run_terminal("timesheet.json") {
        eprint!("Error: {}", e);
        std::process::exit(1);
    }
}
