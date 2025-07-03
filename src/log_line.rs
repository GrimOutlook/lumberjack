use derive_new::new;
use itertools::Itertools;

use crate::{field::Field, field_info::FieldInfo};

#[derive(Debug, Clone, PartialEq, new)]
pub struct LogLine {
    pub fields: Vec<Field>,
}

impl LogLine {
    pub fn raw(&self) -> Vec<&str> {
        self.fields.iter().map(Field::text).collect_vec()
    }
}

impl<'a> FromIterator<&'a str> for LogLine {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let fields = iter
            .into_iter()
            .enumerate()
            .map(|(index, text)| Field {
                field_info: FieldInfo::new(index).into(),
                text: text.into(),
            })
            .collect();

        LogLine { fields }
    }
}

#[cfg(test)]
mod test {
    use crate::{field::Field, field_info::FieldInfo};

    use super::LogLine;

    #[test]
    fn from_iter() {
        let test_array = ["hello", "goodbye"];
        let expected = LogLine {
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
        let actual = LogLine::from_iter(test_array);
        assert_eq!(actual, expected);
    }
}
