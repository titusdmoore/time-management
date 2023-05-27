use std::{
    fs,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

pub struct Parser {
    path: PathBuf,
}

impl Parser {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn parse(&mut self) -> io::Result<&mut Self> {
        let f = fs::File::open(&self.path)?;
        let mut reader = BufReader::new(f);

        for lines in reader.lines() {
            let line = lines?;
            println!("{}", line);
        }

        Ok(self)
    }
}
