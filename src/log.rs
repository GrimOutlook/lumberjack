use std::slice::Iter;

use derive_new::new;
use itertools::Itertools;

use crate::{field::Field, field_info::FieldInfo, log_message::LogMessage};

#[derive(Debug, Clone, PartialEq, new)]
pub struct Log {
    pub messages: Vec<LogMessage>,
}

impl Log {
    pub fn raw(&self) -> Vec<Vec<&str>> {
        self.messages.iter().map(LogMessage::raw).collect_vec()
    }
}

impl FromIterator<Vec<&'static str>> for Log {
    fn from_iter<I: IntoIterator<Item = Vec<&'static str>>>(iter: I) -> Self {
        let messages = iter.into_iter().map(LogMessage::from_iter).collect();

        Log { messages }
    }
}

fn constraint_len_calculator(items: &[Data]) -> (u16, u16, u16) {
    let name_len = items
        .iter()
        .map(Data::name)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let address_len = items
        .iter()
        .map(Data::address)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let email_len = items
        .iter()
        .map(Data::email)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (name_len as u16, address_len as u16, email_len as u16)
}

#[cfg(test)]
mod tests {}

#[cfg(test)]
mod test {
    use crate::{field::Field, field_info::FieldInfo, log::Log, log_message::LogMessage};

    #[test]
    fn constraint_len_calculator() {
        let test_data = vec![
            Data {
                name: "Emirhan Tala".to_string(),
                address: "Cambridgelaan 6XX\n3584 XX Utrecht".to_string(),
                email: "tala.emirhan@gmail.com".to_string(),
            },
            Data {
                name: "thistextis26characterslong".to_string(),
                address: "this line is 31 characters long\nbottom line is 33 characters long"
                    .to_string(),
                email: "thisemailis40caharacterslong@ratatui.com".to_string(),
            },
        ];
        let (longest_name_len, longest_address_len, longest_email_len) =
            crate::constraint_len_calculator(&test_data);

        assert_eq!(26, longest_name_len);
        assert_eq!(33, longest_address_len);
        assert_eq!(40, longest_email_len);
    }

    #[test]
    fn from_iter() {
        let test_messages = vec![vec!["12:10", "message 1"], vec!["13:15", "message 2"]];
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
