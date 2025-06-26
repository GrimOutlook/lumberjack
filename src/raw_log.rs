use crate::{filter::Filter, log_message::LogMessage, parsed_log::ParsedLog};

use anyhow::Result;
use derive_new::new;

#[derive(new)]
struct RawLog {
    // Full, unfiltered, unaltered log text
    #[new(into)]
    log_text: String,
}

impl RawLog {
    pub fn parse_log(&self, filter: Filter) -> Result<ParsedLog> {
        // Split the log based on newlines at first. Some of the lines are not new log messages and
        // a simply continuations of previous messages but we will determine that per log message.
        let lines = self.log_text.split("\n");
        let mut log_lines = vec![];
        for log_line in lines {
            let log_message = Self::parse_message(filter.clone(), log_line)?;
            log_lines.push(log_message);
        }

        Ok(ParsedLog::new(log_lines))
    }

    fn parse_message(filter: Filter, message_text: &str) -> Result<LogMessage> {
        let fields = filter.parse(message_text)?;
        Ok(LogMessage::new(fields))
    }
}

#[cfg(test)]
mod test {

    use crate::filter::Filter;

    use super::RawLog;

    const TEST_LOG_TEXT: &str = "\
16:44:54.572 [main] INFO  this.is.a.test.Main - Start: Main() Module
16:44:54.576 [main] INFO  this.is.a.test.Module - Instantiating Module";

    #[test]
    fn test_parse_log() {
        let expected_test_log = [
            vec![
                "16:44:54.572",
                "[main]",
                "INFO",
                "this.is.a.test.Main",
                "Start: Main() Module",
            ],
            vec![
                "16:44:54.576",
                "[main]",
                "INFO",
                "this.is.a.test.Module",
                "Instantiating Module",
            ],
        ];

        let mut filter = Filter::default();
        filter
            .set_regex(r"(\d{2}:\d{2}:\d{2}\.\d{3})\s+(\[\S+\])\s+(\S+)\s+(\S+)\s+-\s+(.*)")
            .unwrap();
        for i in 1..6 {
            filter.add_field(i);
        }
        let raw_log = RawLog::new(TEST_LOG_TEXT);
        let log = raw_log.parse_log(filter).unwrap();
        itertools::assert_equal(log.raw().iter(), expected_test_log.iter())
    }
}
