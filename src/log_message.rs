use derive_new::new;
use itertools::Itertools;

use crate::{field::Field, field_info::FieldInfo};

#[derive(Debug, Clone, PartialEq, new)]
pub struct LogMessage {
    pub fields: Vec<Field>,
}

impl LogMessage {
    pub fn raw(&self) -> Vec<&str> {
        self.fields.iter().map(Field::text).collect_vec()
    }
}

impl<'a> FromIterator<&'a str> for LogMessage {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let fields = iter
            .into_iter()
            .enumerate()
            .map(|(index, text)| Field {
                field_info: FieldInfo::new(index).into(),
                text: text.into(),
            })
            .collect();

        LogMessage { fields }
    }
}

#[cfg(test)]
mod test {
    use crate::{field::Field, field_info::FieldInfo};

    use super::LogMessage;

    #[test]
    fn from_iter() {
        let test_array = ["hello", "goodbye"];
        let expected = LogMessage {
            fields: vec![
                Field {
                    field_info: FieldInfo::new(0).into(),
                    text: test_array[0].into(),
                },
                Field {
                    field_info: FieldInfo::new(1).into(),
                    text: test_array[1].into(),
                },
            ],
        };
        let actual = LogMessage::from_iter(test_array);
        assert_eq!(actual, expected);
    }
}
