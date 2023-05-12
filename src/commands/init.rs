use std::{fs::create_dir_all, io::Result, path::PathBuf};

pub struct Init {
    pub path: Option<PathBuf>,
}

impl Init {
    pub fn new(path: Option<PathBuf>) -> Self {
        match path {
            Some(path) => Self { path: Some(path) },
            None => Self {
                path: Some(PathBuf::from("~/.local/share")),
            },
        }
    }
    pub fn run(&self) -> Result<()> {
        if let Some(path) = &self.path {
            return create_dir_all(path.join("time_management"));
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Something went wrong initializing the config files.",
        ))
    }
}
