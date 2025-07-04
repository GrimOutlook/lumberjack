use std::{fs::File, io::BufReader};

use crate::log_line::LogLine;

use super::filter::Filter;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct SimpleFilter {
    fields: usize,
}

impl SimpleFilter {
    pub fn new(fields: usize) -> Self {
        SimpleFilter { fields }
    }
}

impl Filter for SimpleFilter {
    fn parse(&self, reader: BufReader<File>) -> Result<Vec<LogLine>> {
        todo!()
    }
}
