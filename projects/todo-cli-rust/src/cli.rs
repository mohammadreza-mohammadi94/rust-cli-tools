use clap::{Parser, Subcommand};
use chrono::NaiveDateTime;
use chrono::TimeZone;

/// Simple CLI To-Do List
#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(about = "Manage your tasks easily from the terminal")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// The task description
        title: String,

        /// Optional due date in format "YYYY-MM-DD HH:MM"
        #[arg(short, long)]
        due: Option<String>,
    },

    /// List all tasks
    List,

    /// Mark a task as completed
    Done {
        /// Task ID to mark as done
        id: u32,
    },

    /// Remove a task
    Remove {
        /// Task ID to remove
        id: u32,
    },
}

pub fn parse_due_date(date_str: &str) -> Option<chrono::DateTime<chrono::Local>> {
    NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M")
        .ok()
        .map(|ndt| chrono::Local.from_local_datetime(&ndt).unwrap())
}
