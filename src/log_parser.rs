use crate::{filter::Filter, log::Log, log_message::LogMessage};

use anyhow::Result;

pub fn parse_log(filter: Filter, full_log_text: &str) -> Result<Log> {
    // Split the log based on newlines at first. Some of the lines are not new log messages and
    // a simply continuations of previous messages but we will determine that per, log message.
    let lines = full_log_text.split("\n");
    let mut log_lines = vec![];
    for log_line in lines {
        let log_message = parse_message(filter.clone(), log_line)?;
        log_lines.push(log_message);
    }

    Ok(Log::new(log_lines))
}

fn parse_message(filter: Filter, message_text: &str) -> Result<LogMessage> {
    let fields = filter.parse(message_text)?;
    Ok(LogMessage::new(fields))
}

fn raw_log(full_log_text: &str) -> Log {
    todo!()
}

#[cfg(test)]
mod test {

    use crate::{filter::Filter, log_parser::parse_log};

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

        let mut filter =
            Filter::new(r"(\d{2}:\d{2}:\d{2}\.\d{3})\s+(\[\S+\])\s+(\S+)\s+(\S+)\s+-\s+(.*)")
                .unwrap();
        for i in 1..6 {
            filter.add_field(i);
        }
        let log = parse_log(filter, TEST_LOG_TEXT).unwrap();
        itertools::assert_equal(log.raw().iter(), expected_test_log.iter())
    }
}
