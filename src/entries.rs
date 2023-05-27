use chrono::{DateTime, Local};

pub struct Entries {
    pub project: Option<String>,
    pub task: Option<String>,
    pub entries: Vec<Entry>,
}

pub struct Entry {
    amount: u32,
    message: Option<String>,
    start: Option<DateTime<Local>>,
    end: Option<DateTime<Local>>,
}

impl Entries {
    pub fn new(project: Option<String>, task: Option<String>) -> Self {
        Self {
            project,
            task,
            entries: Vec::new(),
        }
    }
    pub fn add_entry(
        &mut self,
        amount: u32,
        message: Option<String>,
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) {
        let entry = Entry {
            amount,
            message,
            start,
            end,
        };
        self.entries.push(entry);
    }
}
