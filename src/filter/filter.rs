use std::{fs::File, io::BufReader};

use crate::log_line::LogLine;
use anyhow::Result;
use enum_dispatch::enum_dispatch;

use super::{regex_filter::RegexFilter, simple_filter::SimpleFilter};

#[enum_dispatch(FilterMode)]
pub trait Filter {
    fn parse(&self, reader: BufReader<File>) -> Result<Vec<LogLine>>;
}

#[enum_dispatch]
#[derive(Debug)]
pub enum FilterMode {
    RegexFilter,
    SimpleFilter,
}

impl Default for FilterMode {
    fn default() -> Self {
        FilterMode::SimpleFilter(SimpleFilter::new(0))
    }
}
