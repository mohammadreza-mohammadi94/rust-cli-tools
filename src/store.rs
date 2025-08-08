use crate::task::Task;
use anyhow::Result;
use std::{fs, path::PathBuf};

pub struct Store {
    path: PathBuf,
}

impl Store {
    pub fn new(path: PathBuf) -> Self {
        Store { path }
    }

    pub fn load(&self) -> Result<Vec<Task>> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let data = fs::read_to_string(&self.path)?;
        let tasks: Vec<Task> = serde_json::from_str(&data)?;
        Ok(tasks)
    }

    pub fn save(&self, tasks: &[Task]) -> Result<()> {
        let data = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.path, data)?;
        Ok(())
    }
}
