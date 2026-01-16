# CLI Practice (Rust)

This folder contains small command-line tools written while learning Rust CLI development.

The main focus here is understanding how real CLI programs are structured, including argument parsing, state management, and persistence.

### What this project explores
- `clap` for command parsing
- Subcommands and flags
- JSON-based persistence using `serde`
- Structuring a CLI app into modules
- Ownership and borrowing in real workflows

### Example project
A simple **todo CLI** with:
- user registration
- login / logout
- task creation
- task listing
- task completion
- persistent storage via JSON

### Why JSON?
Well I'm still a beginner and can't afford anything more complex than that, this apps goal was just to understand :
- how data flows through a CLI
- how state is persisted across runs
- how Rust models data safely

### How to run
Each CLI project is a standalone crate.

Example:
```bash
cargo run register alice pass pass
cargo run login alice pass
cargo run add "Buy milk"
cargo run list
cargo run complete 1
```