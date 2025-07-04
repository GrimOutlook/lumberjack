use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

use anyhow::{Context, Result, bail};
use regex::Regex;
use thiserror::Error;

use crate::{field::Field, log_line::LogLine};

use super::filter::Filter;

#[derive(Debug, Error)]
enum ParsingError {
    #[error("Filter does not apply to text: {0}")]
    FilterMismatch(String),
}

#[derive(Clone, Debug)]
pub struct RegexFilter {
    filter_string: String,
    filter: Regex,
}

impl RegexFilter {
    pub fn set_regex(&mut self, filter_string: impl ToString) -> Result<&mut Self> {
        let filter_string = &filter_string.to_string();
        let re = Regex::new(filter_string)
            .with_context(|| format!("Invalid message filter {}", filter_string))?;

        self.filter_string = filter_string.clone();
        self.filter = re;

        Ok(self)
    }
}

impl Filter for RegexFilter {
    fn parse(&self, reader: BufReader<File>) -> Result<Vec<LogLine>> {
        let mut log_lines = Vec::new();
        for line in reader.lines() {
            let line = line.with_context(|| "Failed to read line")?;
            let Some(captures) = self.filter.captures(&line) else {
                bail!(ParsingError::FilterMismatch(line))
            };

            let fields = captures
                .iter()
                .skip(1) // Skip the full match
                .filter_map(|cap| cap.map(|c| c.as_str().to_owned()))
                .enumerate()
                .map(|(index, text)| Field {
                    field_info: Rc::new(crate::field_info::FieldInfo::new(index)),
                    text: text.into(),
                })
                .collect::<Vec<_>>();

            log_lines.push(LogLine { fields });
        }

        Ok(log_lines)
    }
}
