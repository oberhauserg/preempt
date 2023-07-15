use crate::timeblock::TimeBlock;

/// Various file operations.
use super::context::Context;
use super::schedule::build_schedule;
use super::task::Task;

use chrono::Utc;
use directories::ProjectDirs;
use serde;
use serde_yaml;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PreemptApp {
    tasks: Vec<Task>,
    contexts: Vec<Context>,
}

impl PreemptApp {
    pub fn new() -> PreemptApp {
        PreemptApp {
            tasks: vec![],
            contexts: vec![],
        }
    }

    pub fn add_task(&mut self, task: Task) -> Result<(), &'static str> {
        if self.get_task(&task.name).is_none() {
            self.tasks.push(task);
            return Ok(());
        } else {
            Err("Task already exists")
        }
    }

    pub fn get_task(&self, name: &String) -> Option<&Task> {
        for task in &self.tasks {
            if task.name.to_lowercase() == name.to_lowercase() {
                return Some(&task);
            }
        }
        None
    }

    pub fn add_context(&mut self, context: Context) -> Result<(), &'static str> {
        if self.get_context(&context.name).is_none() {
            self.contexts.push(context);
            return Ok(());
        } else {
            Err("Context already exists")
        }
    }

    pub fn get_context(&self, name: &String) -> Option<&Context> {
        for context in &self.contexts {
            if context.name.to_lowercase() == name.to_lowercase() {
                return Some(&context);
            }
        }
        None
    }

    pub fn build_schedule(&self) -> Vec<TimeBlock> {
        build_schedule(
            &self.contexts,
            &self.tasks,
            TimeBlock::new(
                chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
                Utc::now().date_naive(),
                Utc::now().date_naive(),
            ),
        )
    }
}

pub fn get_dir() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "grant", "preempt")
}

pub fn save(data: &PreemptApp) -> Result<(), &'static str> {
    let serialized_data = serde_yaml::to_string(data).unwrap();

    if let Some(proj_dirs) = get_dir() {
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(&data_dir).map_err(|_| "Couldn't create directory")?;

        let file_path = data_dir.join("preempt_data.yaml"); // specify the file name
        let path = Path::new(&file_path);

        let mut file = match OpenOptions::new().write(true).create(true).open(&path) {
            Err(_) => return Err("Couldn't open file"),
            Ok(file) => file,
        };

        match file.write_all(serialized_data.as_bytes()) {
            Err(_) => return Err("Couldn't write to file"),
            Ok(_) => (),
        }
    } else {
        return Err("Couldn't get directory");
    }

    Ok(())
}

pub fn load() -> Result<PreemptApp, Box<dyn std::error::Error>> {
    if let Some(proj_dirs) = get_dir() {
        let data_dir = proj_dirs.data_dir();
        let file_path = data_dir.join("preempt_data.yaml");
        let path = Path::new(&file_path);
        let mut file = File::open(&path)?;

        let mut serialized_data = String::new();
        file.read_to_string(&mut serialized_data)?;

        let deserialized_data: PreemptApp = serde_yaml::from_str(&serialized_data)?;

        Ok(deserialized_data)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Couldn't get directory",
        )))
    }
}
