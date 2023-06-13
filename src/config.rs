use crate::errors::Errors;
use crate::utils::Months;
use chrono::{Datelike, Local};
use dirs::home_dir;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub config_path: Option<PathBuf>,
    pub work_path: Option<PathBuf>,
}

#[derive(Deserialize)]
struct TomlConfig {
    file_path: String,
}

impl Config {
    pub fn init() -> Self {
        let mut home: Option<PathBuf> = None;
        let mut file_path: Option<PathBuf> = None;

        if let Some(mut home_path) = home_dir() {
            home_path.push(".config/time-management/config.toml");

            if home_path.exists() {
                let toml_config: TomlConfig =
                    toml::from_str(read_to_string(&home_path).unwrap().as_ref()).unwrap();

                file_path = Some(PathBuf::from(toml_config.file_path));
                home = Some(home_path);
            }
        }

        Config {
            config_path: home,
            work_path: file_path,
        }
    }

    pub fn today_tuple(&self) -> Result<(u8, u8), Errors> {
        let now = Local::now().date_naive();
        let month = match now.month().try_into() {
            Ok(month) => month,
            Err(e) => {
                return Err(Errors::TryFrom(e));
            }
        };
        let day = match now.day().try_into() {
            Ok(day) => day,
            Err(e) => {
                return Err(Errors::TryFrom(e));
            }
        };

        Ok((month, day))
    }

    pub fn today_path(&self) -> Result<(PathBuf, String), Errors> {
        let now = Local::now().date_naive();
        let month: String = Months::get_month(now.month() as usize).to_string();
        let day = now.day().to_string();

        if let Some(work_path) = &self.work_path {
            return Ok((
                work_path.join(format!("{}", month)),
                format!("{}.toml", day),
            ));
        }
        Err(Errors::Error("No work path found".to_string()))
    }
}
