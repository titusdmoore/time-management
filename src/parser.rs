use std::{fs, io, path::PathBuf};

pub struct Parser {
    path: PathBuf,
    string_content: Option<String>,
}

impl Parser {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            string_content: None,
        }
    }

    pub fn parse(&mut self) -> io::Result<&mut Self> {
        self.string_content = Some(fs::read_to_string(&self.path)?);

        Ok(self)
    }

    pub fn 
}
