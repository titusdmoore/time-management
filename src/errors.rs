use std::num::{ParseIntError, TryFromIntError};

#[derive(Debug)]
pub enum Errors {
    Io(std::io::Error),
    Parse(ParseIntError),
    TryFrom(TryFromIntError),
    Error(String),
}
