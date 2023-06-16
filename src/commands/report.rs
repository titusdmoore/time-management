use std::ffi::OsString;
use std::{fs, path::PathBuf};

use crate::config::Config;
use crate::errors::Errors;
use crate::time_log::TimeLog;
use crate::utils::Months;

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
    // if start_tuple == end_tuple {
    //     let (path, file) = config.today_path().unwrap();
    //     time_logs.push(TimeLog::from(path.join(file)).unwrap());
    //
    //     return time_logs;
    // }

    let paths = walk_diff_between_tuples(config, start_tuple, end_tuple).unwrap();

    for path in paths {
        time_logs.push(TimeLog::from(path).unwrap());
    }

    time_logs
}

fn day_arg_to_tuple(day: String) -> Result<(u8, u8), Errors> {
    let day: Vec<&str> = day.split('/').collect();
    let month = Months::from(day[0]).to_u8();

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

    // This does not support years yet
    // TODO: Support years
    for month in start.0..=end.0 {
        // replace with read dir iterator
        if let Ok(dir) = fs::read_dir(
            &config
                .work_path
                .as_ref()
                .unwrap()
                .join(Months::get_month(usize::try_from(month).unwrap()).to_string()),
        ) {
            for file in dir {
                if let Ok(file) = file {
                    if month == end.0 && name_to_u32(file.file_name())? > end.1.into() {
                        continue;
                    }

                    paths.push(file.path());
                }
            }
        }
    }

    Ok(paths)
}

fn name_to_u32(name: OsString) -> Result<u32, Errors> {
    let name = match name.into_string() {
        Ok(name) => name,
        Err(e) => {
            return Err(Errors::Error(format!(
                "Could not convert OsString to String: {:?}",
                e
            )));
        }
    };
    let name = name.replace(".toml", "");
    let name = match name.parse::<u32>() {
        Ok(name) => name,
        Err(e) => {
            return Err(Errors::Parse(e));
        }
    };

    Ok(name)
}
