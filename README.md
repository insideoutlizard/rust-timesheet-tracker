# Rust Project Tracker


A fast, lightweight, and cross‑platform command‑line tool for logging and reviewing **any kind of project work** — from creative writing to home renovations.  
Built in Rust for reliability and speed, it’s designed to grow from a simple tracker into a full‑featured personal project management suite.

────────────────────────────────────────────

## ✦ Features

- Track sessions with **date**, **project name**, **description**, and **duration** (in minutes)
- Persistent storage in a local JSON file
- View all entries in a clean, readable list
- Summarize total time spent on a specific project
- Runs anywhere Rust can compile

────────────────────────────────────────────

## ⌘ Installation

1. **Clone the repo**
    ```bash
    git clone https://github.com/insideoutlizard/rust-timesheet-tracker
    cd rust-timesheet-tracker
    ```
2. **Build and run**
    ```bash
    cargo run
    ```

────────────────────────────────────────────

## ⚙ Usage

*(CLI interface in progress — interactive commands coming soon)*

────────────────────────────────────────────

## ☰ Roadmap

### Short‑Term Goals
- [x] Implement `load_entries()` function
- [ ] Add `save_entries()` function
- [ ] Implement `add_entry()` with interactive prompts
- [ ] Pretty‑print tables for `list_entries()`
- [ ] Filter entries by date range
- [ ] Export to CSV

### Long‑Term / Stretch Goals
- [ ] GUI interface (Tauri, egui, or web‑based dashboard)
- [ ] More file types for export (TOML, YAML, Excel, PDF)
- [ ] Calendar view for entries (month/week/day navigation)
- [ ] Email and push notifications (reminders, weekly summaries)
- [ ] Theming (light/dark mode, custom color schemes)
- [ ] Cloud sync between devices
- [ ] User authentication for multi‑user tracking
- [ ] Tagging system for categorizing work
- [ ] Data visualization (charts/graphs of time spent)
- [ ] Integration with external APIs (GitHub activity, Jira, etc.)

────────────────────────────────────────────

## ⎘ Data Format

Entries are stored in **timesheet.json** in this format:

```json
[
  {
    "date": "2025-09-07T14:00:00Z",
    "project": "Rust Project Tracker",
    "description": "Implemented load_entries",
    "duration": 90
  }
]
```

────────────────────────────────────────────

## ⚖ License

[Not implemented]