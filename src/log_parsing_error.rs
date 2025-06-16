use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct LogParsingError<'a> {
    message: &'a str,
}

impl<'a> Error for LogParsingError<'a> {}

impl<'a> Display for LogParsingError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
