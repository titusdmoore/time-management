use std::path::PathBuf;

use crate::config::Config;
use crate::errors::Errors;
use crate::time_log::TimeLog;

pub fn logs_from_range(
    config: &Config,
    // Project and task are hidden because eventually they will be used to filter the logs.
    _project: &Option<String>,
    _task: &Option<String>,
    start: &Option<String>,
    end: &Option<String>,
) -> Vec<TimeLog> {
    let mut time_logs: Vec<TimeLog> = Vec::new();

    let start_tuple = match start {
        Some(start) => match day_arg_to_tuple(start.to_string()) {
            Ok(tuple) => tuple,
            Err(e) => {
                println!("Error: {:?}", e);
                std::process::exit(1);
            }
        },
        None => match config.today_tuple() {
            Ok(tuple) => tuple,
            Err(e) => {
                println!("Error: {:?}", e);
                std::process::exit(1);
            }
        },
    };
    let end_tuple = match end {
        Some(end) => match day_arg_to_tuple(end.to_string()) {
            Ok(tuple) => tuple,
            Err(e) => {
                println!("Error: {:?}", e);
                std::process::exit(1);
            }
        },
        None => match config.today_tuple() {
            Ok(tuple) => tuple,
            Err(e) => {
                println!("Error: {:?}", e);
                std::process::exit(1);
            }
        },
    };

    // They just want today
    if start_tuple == end_tuple {
        let (path, file) = config.today_path().unwrap();
        time_logs.push(TimeLog::from(path.join(file)).unwrap());
    }

    walk_diff_between_tuples(config, start_tuple, end_tuple).unwrap();

    time_logs
}

fn day_arg_to_tuple(day: String) -> Result<(u8, u8), Errors> {
    let day: Vec<&str> = day.split('/').collect();
    let month = match day[0].parse::<u8>() {
        Ok(day) => day,
        Err(e) => {
            return Err(Errors::Parse(e));
        }
    };

    let day = match day[1].parse::<u8>() {
        Ok(day) => day,
        Err(e) => {
            return Err(Errors::Parse(e));
        }
    };

    Ok((month, day))
}

fn walk_diff_between_tuples(
    config: &Config,
    start: (u8, u8),
    end: (u8, u8),
) -> Result<Vec<PathBuf>, Errors> {
    let mut paths: Vec<PathBuf> = Vec::new();

    // convert start.0 to month string (which is what the dir name is in TM working dir)
    // loop each month between start.0 end.0
    // walk the day files in each month dir while < 31 && file exists (besides the start.0 month,
    // which should start at start.1)
    println!("{:?}", config);

    Err(Errors::Error("Not implemented yet".to_string()))
}
