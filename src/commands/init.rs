use dirs::home_dir;
use std::io::prelude::*;
use std::{
    fs::{create_dir_all, File},
    io::Result,
    path::PathBuf,
};

pub struct Init {
    pub path: Option<PathBuf>,
}

impl Init {
    pub fn new(path: Option<PathBuf>) -> Self {
        match path {
            Some(path) => Self { path: Some(path) },
            None => {
                if let Some(path) = home_dir() {
                    return Self {
                        path: Some(path.join(".local/share")),
                    };
                } else {
                    return Self {
                        path: Some(PathBuf::from("/usr/local/share")),
                    };
                }
            }
        }
    }
    // I need to add a config file that has the path used, then I need to add a way for the path to
    // get set to a state obj that can be used by the other commands.
    // I don't like this name
    pub fn run(&self) -> Result<()> {
        let root_path = home_dir().unwrap();
        create_dir_all(root_path.join(".config/time-management"))?;

        if let Some(path) = &self.path {
            create_dir_all(path.join("time-management"))?;

            match File::create(
                root_path
                    .join(".config/time-management")
                    .join("config.toml"),
            ) {
                Ok(mut file) => {
                    file.write_all(
                        format!("file_path = {}", path.display().to_string()).as_bytes(),
                    )?;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
