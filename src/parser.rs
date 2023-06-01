use regex::Regex;
use std::{
    fs,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use crate::time_log::TimeLog;
use chrono::{DateTime, Local, TimeZone};

pub struct Parser {
    path: PathBuf,
    entries: Vec<TimeLog>,
}

impl Parser {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            entries: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> io::Result<&mut Self> {
        let f = fs::File::open(&self.path)?;
        let reader = BufReader::new(f);
        let mut raw_entry: Vec<String> = Vec::new();

        for lines in reader.lines() {
            let line = lines?;

            if Self::is_project_head(&line) {
                if raw_entry.len() > 0 {
                    // bad error handling, refactor
                    // let time_log = TimeLog::try_parse(&raw_entry).unwrap();
                    // try parse into entries struct
                    // if success, push into entries
                    // else clear vec and print error
                    // continue
                    println!("Reached end of entry: {:?}", raw_entry);
                    TimeLog::try_parse(&raw_entry).unwrap();
                    raw_entry.clear();
                }
            }
            if !line.is_empty() {
                raw_entry.push(line.trim().to_string());
            }
        }

        println!("{:?}", Local::now());
        println!("Reached end of file: {:?}", raw_entry);

        Ok(self)
    }

    fn is_project_head(line: &String) -> bool {
        let re = Regex::new(r"^\[\s*\w+\s*(?:\:\s*\w+\s*)?\]").unwrap();
        re.is_match(line)
    }
}
