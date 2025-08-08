use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Local};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A colorful CLI To-Do list in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        description: String,
        #[arg(short, long)]
        due: Option<String>, // Format: YYYY-MM-DD
    },
    /// List all tasks
    List,
    /// Mark a task as completed
    Done { id: usize },
    /// Delete a task
    Delete { id: usize },
}

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
    due: Option<NaiveDate>,
}

const FILE: &str = "tasks.json";

fn load_tasks() -> Vec<Task> {
    if PathBuf::from(FILE).exists() {
        let data = fs::read_to_string(FILE).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(FILE, data).expect("Failed to save tasks");
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { description, due } => {
            let due_date = due.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
            tasks.push(Task { description, completed: false, due: due_date });
            save_tasks(&tasks);
            println!("{}", "✅ Task added!".green().bold());
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("{}", "📭 No tasks found".yellow());
            } else {
                println!("{}", "📝 To-Do List:".bold().underline());
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.completed {
                        "✔ Done".green().bold()
                    } else {
                        "⏳ Pending".cyan().bold()
                    };

                    let mut line = format!("{}: {}", i, task.description);

                    if let Some(due) = task.due {
                        let today = Local::now().date_naive();
                        if !task.completed && due < today {
                            line.push_str(&format!(" (Due: {})", due.format("%Y-%m-%d")).red().to_string());
                        } else {
                            line.push_str(&format!(" (Due: {})", due.format("%Y-%m-%d")).bold().to_string());
                        }
                    }

                    println!("{} - {}", line, status);
                }
            }
        }
        Commands::Done { id } => {
            if id < tasks.len() {
                tasks[id].completed = true;
                save_tasks(&tasks);
                println!("{}", "🎉 Task marked as completed!".green().bold());
            } else {
                println!("{}", "⚠ Task ID not found".red());
            }
        }
        Commands::Delete { id } => {
            if id < tasks.len() {
                tasks.remove(id);
                save_tasks(&tasks);
                println!("{}", "🗑 Task deleted".red().bold());
            } else {
                println!("{}", "⚠ Task ID not found".red());
            }
        }
    }
}
