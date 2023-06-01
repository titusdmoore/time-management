use chrono::{DateTime, Local};

pub struct TimeLog {
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

impl TimeLog {
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
    pub fn try_parse(raw_time_log: &Vec<String>) -> Result<TimeLog, ()> {
        let mut time_log = Self::try_parse_time_log_string(&raw_time_log[0])?;

        for line in raw_time_log.iter().skip(1) {
            let mut val_vec: Vec<(usize, usize)> = Vec::new();
            let ch_bytes = line.trim().as_bytes();

            // Set start to 1 to skip initial '-'
            let mut i = 1;
            while i < ch_bytes.len() {
                println!("{:?}", ch_bytes[i] as char);
                match ch_bytes[i] as char {
                    '-' | '[' | ']' | ':' | '"' => {
                        println!("in");
                        let vec_len = val_vec.len();

                        // This is inside of the amount, seperating the two numbers
                        if vec_len == 1 {
                            i += 1;
                            continue;
                        }

                        if vec_len != 0 {
                            println!("Pushing: {:?}", val_vec[vec_len - 1]);
                            val_vec[vec_len - 1].1 = i - 1;
                        }
                        val_vec.push((i + 1, usize::MAX));

                        i += 1;
                        continue;
                    }
                    _ => {
                        i += 1;
                        continue;
                    }
                }
            }
            println!("{:?}", val_vec);

            // for (start, end) in val_vec {
            //     let val = &line[start..end];
            //     println!("{}", val);
            // }
        }

        Ok(time_log)
    }
    fn try_parse_time_log_string(raw_time_log: &String) -> Result<TimeLog, ()> {
        let mut project = String::new();
        let mut task: Option<String> = None;
        let mut is_project = true;

        for ch in raw_time_log.chars() {
            if ch == '[' || ch == ' ' {
                continue;
            }

            if ch == ']' {
                break;
            }

            if ch == ':' {
                is_project = false;
                continue;
            }

            if is_project {
                project.push(ch);
            } else {
                match task.as_mut() {
                    Some(t) => t.push(ch),
                    None => task = Some(ch.to_string()),
                }
            }
        }

        if project.is_empty() {
            return Err(());
        }

        Ok(TimeLog::new(Some(project), task))
    }
}
