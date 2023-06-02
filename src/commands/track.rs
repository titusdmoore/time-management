use crate::config::Config;
use crate::time_log::Entry;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub struct Track {
    pub today_path: Option<(PathBuf, String)>,
    pub entry: Entry,
}

impl Track {
    pub fn new(config: &Config, entry: Entry) -> Self {
        Self {
            today_path: config.today_path().ok(),
            entry,
        }
    }
    pub fn run(&self) {
        if let Some((path, file_name)) = &self.today_path {
            fs::create_dir_all(path).unwrap();
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(path.join(file_name))
                .unwrap();

            writeln!(file, "{}", self.entry.to_log_string()).unwrap();
        }
    }
}
