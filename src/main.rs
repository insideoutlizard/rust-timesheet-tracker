mod entries;
mod terminal;

use terminal::run_terminal;

fn main() {
    if let Err(e) = run_terminal("timesheet.json") {
        eprint!("Error: {}", e);
        std::process::exit(1);
    }
}
