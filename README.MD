# todo-cli

A professional, colorful CLI To-Do List application written in Rust.

Manage your tasks easily from the terminal with a clean interface, persistent JSON storage, and color-coded status indicators.

---

## Features

- Add tasks quickly from the command line
- List all tasks with clear colored status and strikethrough for completed items
- Mark tasks as done
- Remove tasks by ID
- Persistent storage of tasks in a local JSON file (`todo.json`)
- Colored output for better readability and user experience

---

## Installation

Make sure you have Rust and Cargo installed. Then clone the repo or download the source code, navigate to the project directory and run:

```bash
cargo install --path .
````

This installs the CLI globally, and the executable will be available as `todo_cli` (Windows) or `todo_cli` (Linux/Mac).

---

## Usage

Run the CLI commands using the installed executable (`todo_cli` by default):

### Add a new task

```bash
todo_cli add "Buy groceries"
```

### List all tasks

```bash
todo_cli list
```

Example output:

```
📋 To-Do List:
#1 • Buy groceries
#2 ✔ Finish Rust project
```

### Mark a task as done

```bash
todo_cli done 1
```

### Remove a task

```bash
todo_cli remove 1
```

---

## Notes

* Tasks are stored in `todo.json` in the current working directory.
* Task IDs start from 1 when listing or referencing.

---

## Customize

If you want to rename the executable to something like `todo-cli` or `todo`, change the `[package] name` field in `Cargo.toml` and reinstall.

---

## License

MIT License © Your Name

---

## Acknowledgments

* Uses [clap](https://crates.io/crates/clap) for command-line parsing
* Uses [serde](https://crates.io/crates/serde) + [serde\_json](https://crates.io/crates/serde_json) for JSON serialization
* Uses [colored](https://crates.io/crates/colored) for terminal color output

