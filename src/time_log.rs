use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io::Error};

use crate::errors::Errors;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeLog {
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub project: String,
    pub task: Option<String>,
    pub amount: u32,
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
    pub message: Option<String>,
}

impl TimeLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn total_time(&self) -> u32 {
        let mut total_time = 0;

        for entry in &self.entries {
            total_time += entry.amount;
        }

        total_time
    }

    pub fn from(path: PathBuf) -> Result<Self, Errors> {
        let file_str = Self::read_to_string_or_create(path)?;

        // If file is empty, we may have just created it, we don't want this to error, so we do a
        // sneaky check here.
        if file_str.is_empty() {
            return Ok(Self::new());
        }

        match toml::from_str(&file_str) {
            Ok(time_log) => Ok(time_log),
            Err(e) => {
                println!("Error: Unable to parse time log file.\n{}", e);
                Err(Errors::Error(format!(
                    "Unable to parse syntax of time log: {}",
                    e
                )))
            }
        }
    }

    fn read_to_string_or_create(path: PathBuf) -> Result<String, Errors> {
        match fs::read_to_string(&path) {
            Ok(file_str) => Ok(file_str),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    match fs::write(&path, "") {
                        Ok(_) => Ok("".to_string()),
                        Err(e) => Err(Errors::Io(e)),
                    }
                } else {
                    Err(Errors::Io(e))
                }
            }
        }
    }
}

impl Entry {
    pub fn new(
        project: String,
        task: Option<String>,
        amount: u32,
        start: DateTime<Local>,
        end: Option<DateTime<Local>>,
        message: Option<String>,
    ) -> Self {
        Self {
            project,
            task,
            amount,
            start,
            end,
            message,
        }
    }

    pub fn to_log_string(&self) -> String {
        let mut log_string = String::new();

        log_string.push_str(&format!("[[entries]]\nproject = \"{}\"\n", &self.project));

        if let Some(task) = &self.task {
            log_string.push_str(&format!("task = \"{}\"\n", &task));
        }

        log_string.push_str(&format!("amount = {}\n", self.amount));

        if let Some(message) = &self.message {
            log_string.push_str(&format!("message = \"{}\"\n", &message));
        }

        log_string.push_str(&format!("start = \"{}\"\n", self.start));

        log_string
    }

    pub fn to_report_string(&self) -> String {
        format!(
            "- [{}:{} - {}] {}",
            self.project,
            self.task.clone().unwrap_or("".to_string()),
            Entry::to_string_time(self.amount),
            self.message.clone().unwrap_or("".to_string())
        )
    }

    pub fn to_string_time(amount: u32) -> String {
        let hours = amount / 60;
        let minutes = amount % 60;
        let mut minutes_str = minutes.to_string();

        if minutes < 10 {
            minutes_str = format!("0{}", minutes);
        }

        format!("{}:{}", hours, minutes_str)
    }

    pub fn to_raw_time(amount: String) -> u32 {
        let amount: Vec<&str> = amount.split(':').collect();

        let mut total_minutes: u32 = 0;

        for (i, time) in amount.iter().enumerate() {
            let time: u32 = time.parse().unwrap();

            match i {
                0 => {
                    total_minutes += time * 60;
                }
                1 => {
                    total_minutes += time;
                }
                _ => {}
            }
        }

        total_minutes
    }

    pub fn parse_project_task(input: String) -> (String, Option<String>) {
        let input: Vec<&str> = input.split('/').collect();

        let project = input[0].to_string();
        let task = if input.len() > 1 {
            Some(input[1].to_string())
        } else {
            None
        };

        (project, task)
    }
}
