use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Local>,
    pub due_date: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(id: u32, title: String, due_date: Option<DateTime<Local>>) -> Self {
        Task {
            id,
            title,
            completed: false,
            created_at: Local::now(),
            due_date,
        }
    }
}
