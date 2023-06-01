use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entries {
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub project: String,
    pub task: Option<String>,
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
    pub message: Option<String>,
}
