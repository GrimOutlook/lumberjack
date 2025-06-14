use crate::{log::Log, log_message::LogMessage};

pub struct LogParser {}
impl LogParser {
    pub fn parse_log(text: &str) -> Log {
        todo!()
    }

    fn parse_message() -> LogMessage {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{field::Field, field_info::FieldInfo, log::Log, log_message::LogMessage};

    use super::LogParser;

    const TEST_LOG_TEXT: &str = r#"
16:44:54.572 [main] INFO  this.is.a.test.Main - Start: Main() Module
16:44:54.576 [main] INFO  this.is.a.test.Module - Instantiating Module
"#;

    // #[test]
    // fn parse_log() {
    //     let test_log: Log = Log::from_iter([
    //         [ ],
    //     ]);
    //
    //     let log = LogParser::parse_log(TEST_LOG_TEXT);
    //     assert_eq!()
    // }
}
