# Rust Timesheet Tracker

A simple command-line tool for loggin and reviewing things that aren't programming sessions. This is mainly intended as a learning project to explore Rust programming.

---

## Features

- **Track sessions** with date, project name, description, and duration (in minutes)
- **Persistent storage** in a local JSON file
- **View all entries** in a clean list
- **Summarize total time** spent on a specific project
- **Cross-platform** - runs anywhere Rust can

---

## Installation

1. **Clone the repo**
    ```bash
    git clone https://github.com/insideoutlizard/rust-timesheet-tracker
    cd rust-timesheet-tracker
    ```
2. **Build and run**
    ```bash
    cargo run
    ```

## Usage

[Not implemented]

## Data Format

Entries are stored in **timesheet.json** in this format:

```Json
[
  {
    "date": "2025-09-07T14:00:00Z",
    "project": "Rust Timesheet",
    "description": "Implemented load_entries",
    "duration": 90
  }
]
```


## 🚀 Roadmap

### Short‑Term Goals
- [x] Implement `load_entries()` function
- [ ] Add `save_entries()` function
- [ ] Implement `add_entry()` with interactive prompts
- [ ] Pretty‑print tables for `list_entries()`
- [ ] Filter entries by date range
- [ ] Export to CSV

### Long‑Term / Stretch Goals
- [ ] GUI interface
- [ ] More file types for export (TOML, YAML, Excel, PDF)
- [ ] Calendar view for entries (month/week/day navigation)
- [ ] Email and push notifications (reminders to log time, weekly summaries)
- [ ] Theming (light/dark mode, custom color schemes)
- [ ] Cloud sync between devices
- [ ] User authentication for multi‑user tracking
- [ ] Tagging system for categorizing work
- [ ] Data visualization (charts/graphs of time spent)
- [ ] Integration with external APIs (GitHub activity, Jira, etc.)


# License

[Not implemented]