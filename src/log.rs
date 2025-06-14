use crate::{field::Field, field_info::FieldInfo, log_message::LogMessage};

#[derive(Debug, Clone, PartialEq)]
pub struct Log {
    pub messages: Vec<LogMessage>,
}

impl FromIterator<Vec<&'static str>> for Log {
    fn from_iter<I: IntoIterator<Item = Vec<&'static str>>>(iter: I) -> Self {
        let messages = iter.into_iter().map(LogMessage::from_iter).collect();

        Log { messages }
    }
}

#[cfg(test)]
mod test {
    use crate::{field::Field, field_info::FieldInfo, log::Log, log_message::LogMessage};

    #[test]
    fn from_iter() {
        let test_messages = vec![vec!["1", "message 1"], vec!["2", "message 2"]];
        let expected = Log {
            messages: vec![
                LogMessage {
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
                LogMessage {
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
