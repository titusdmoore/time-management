use crate::config::Config;
use chrono::{Datelike, Local};

use crate::utils::Months;

pub struct Track {
    pub project: String,
    pub task: String,
    pub message: Option<String>,
    pub amount: u32,
}

impl Track {
    pub fn new(project_task: String, message: Option<String>, amount: String) -> Self {
        let project_task: Vec<&str> = project_task.split("/").collect();

        Self {
            project: project_task[0].to_string(),
            task: project_task[1].to_string(),
            message,
            amount: Track::to_raw_time(amount),
        }
    }
    pub fn run(&self, config: &Config) {
        let now = Local::now().date_naive();
        let month: String = Months::get_month(now.month() as usize).to_string();
        let day = now.day().to_string();

        if let Some(work_path) = &config.work_path {
            println!(
                "Work Path: {}",
                work_path.join(format!("{}/{}.txt", month, day)).display()
            );
        }

        println!(
            "[{} : {}]\n\t- [{}] {}",
            self.project,
            self.task,
            Track::to_string_time(self.amount),
            self.message.as_ref().unwrap()
        );
    }
    pub fn to_raw_time(amount: String) -> u32 {
        let amount: Vec<&str> = amount.split(":").collect();

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

    pub fn to_string_time(amount: u32) -> String {
        let hours = amount / 60;
        let minutes = amount % 60;

        format!("{}:{}", hours, minutes)
    }
}
