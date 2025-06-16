use crate::{
    field_info::FieldInfo, filter::Filter, log::Log, log_message::LogMessage,
    log_parsing_error::LogParsingError,
};

pub fn parse_log(filter: Filter, full_log_text: &str) -> Result<Log, LogParsingError> {
    // Split the log based on newlines at first. Some of the lines are not new log messages and
    // a simply continuations of previous messages but we will determine that per, log message.
    let lines = full_log_text.split("\n");
    for log_line in lines {
        let log_message = parse_message(filter.clone(), log_line);
    }
    todo!()
}

fn parse_message(filter: Filter, message_text: &str) -> Result<LogMessage, LogParsingError> {
    todo!()
}

fn raw_log(full_log_text: &str) -> Log {
    todo!()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{field_info::FieldInfo, filter::Filter, log_parser::parse_log};

    const TEST_LOG_TEXT: &str = r#"
16:44:54.572 [main] INFO  this.is.a.test.Main - Start: Main() Module
16:44:54.576 [main] INFO  this.is.a.test.Module - Instantiating Module
"#;

    #[test]
    fn test_parse_log() {
        let expected_test_log = [
            vec![
                "16:44:54.572",
                "[main]",
                "INFO",
                "this.is.a.test.Main",
                "-",
                "Start: Main() Module",
            ],
            vec![
                "16:44:54.576",
                "[main]",
                "INFO",
                "this.is.a.test.Module",
                "-",
                "Instantiating Module",
            ],
        ];

        let mut filter =
            Filter::new(r"(\d{2}:\d{2}:\d{2}\.\d{3})\s+(\[\S+\])\s+(\S+)\s+(\S+)\s+-\s+(.*)");
        for i in 1..5 {
            filter.add_field(i);
        }
        let log = parse_log(filter, TEST_LOG_TEXT).unwrap();
        itertools::assert_equal(log.raw().iter(), expected_test_log.iter())
    }
}
