use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    rc::Rc,
};

use anyhow::{Context, Result};
use getset::Getters;
use itertools::Itertools;

use crate::{field_info::FieldInfo, log_line::LogLine, regex_filter::RegexFilter};

#[derive(Debug, Clone, Getters, PartialEq)]
pub struct Log {
    #[getset(get = "pub")]
    filepath: PathBuf,

    #[getset(get = "pub")]
    lines: Vec<LogLine>,

    #[getset(get = "pub")]
    field_info: Vec<Rc<FieldInfo>>,
}

impl Log {
    pub fn new(filepath: PathBuf) -> Result<Self> {
        Self::parse(filepath, RegexFilter::default())
    }

    fn parse(filepath: PathBuf, filter: RegexFilter) -> Result<Self> {
        let filepath_os_str = filepath.clone().into_os_string();
        let filepath_str = filepath_os_str.as_os_str().to_str().unwrap();

        let file = File::open(filepath.clone())
            .with_context(|| format!("Failed to open file {}", filepath_str))?;
        let reader = BufReader::new(file);

        let mut lines = vec![];
        for (index, line) in reader.lines().enumerate() {
            let line =
                line.with_context(|| format!("Failed to read line: {}:{}", filepath_str, index))?;

            // Remove any empty lines in file.
            let parsed_line_opt = match Self::parse_line(filter.clone(), &line).with_context(|| {
                format!(
                    "Failed to parse line from file {} on line {}",
                    filepath_str, index
                )
            }) {
                Ok(line_opt) => line_opt,
                Err(e) => match e {},
            };
            match parsed_line_opt {
                Some(log_line) => lines.push(log_line),
                None => {
                    // If the returned line is None, it means that the line is empty and should not be added to the parsed log
                    continue;
                }
            };
        }

        Ok(Log {
            filepath,
            lines: lines.clone(),
            field_info: lines.first().map_or_else(
                Vec::new, // If there are no lines, return an empty field_info
                |line| line.fields.iter().map(|f| f.field_info.clone()).collect(),
            ),
        })
    }

    fn parse_line(filter: RegexFilter, message_text: &str) -> Result<Option<LogLine>> {
        // Check if log message is empty
        if message_text.trim().is_empty() {
            return Ok(None);
        }

        let fields = filter.parse(message_text)?;
        Ok(Some(LogLine::new(fields)))
    }

    pub fn field_names(&self) -> Option<Vec<Rc<str>>> {
        self.field_info
            .iter()
            .map(|field_info| field_info.name.clone())
            .collect::<Option<Vec<Rc<str>>>>()
    }

    // TODO: Remove this method if not used by first release
    #[allow(unused)]
    pub fn raw(&self) -> Vec<Vec<&str>> {
        self.lines.iter().map(LogLine::raw).collect_vec()
    }
}

// NOTE: This should only be used for testing purposes
impl FromIterator<Vec<&'static str>> for Log {
    fn from_iter<I: IntoIterator<Item = Vec<&'static str>>>(iter: I) -> Self {
        let messages = iter.into_iter().map(LogLine::from_iter).collect();

        Log {
            filepath: "".into(),
            lines: messages,
            field_info: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{field::Field, field_info::FieldInfo, log::Log, log_line::LogLine};

    #[test]
    fn from_iter() {
        let test_messages = vec![vec!["12:10", "message 1"], vec!["13:15", "message 2"]];
        let expected = Log {
            filepath: "".into(),
            field_info: vec![FieldInfo::new(0).into(), FieldInfo::new(1).into()],
            lines: vec![
                LogLine {
                    fields: vec![
                        Field {
                            field_info: FieldInfo::new(0).into(),
                            text: test_messages[0][0].into(),
                        },
                        Field {
                            field_info: FieldInfo::new(1).into(),
                            text: test_messages[0][1].into(),
                        },
                    ],
                },
                LogLine {
                    fields: vec![
                        Field {
                            field_info: FieldInfo::new(0).into(),
                            text: test_messages[1][0].into(),
                        },
                        Field {
                            field_info: FieldInfo::new(1).into(),
                            text: test_messages[1][1].into(),
                        },
                    ],
                },
            ],
        };

        let actual = Log::from_iter(test_messages);
        assert_eq!(expected, actual);
    }
}
