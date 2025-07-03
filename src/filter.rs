use std::{fmt::Display, rc::Rc};

use anyhow::{Context, Result, bail};
use regex::Regex;
use thiserror::Error;

use crate::{field::Field, field_info::FieldInfo};

#[derive(Debug, Error)]
enum ParsingError {
    #[error("Filter is not set")]
    FilterNotSet,
    #[error("Filter does not apply to text: {0}")]
    FilterMismatch(String),
}

#[derive(Clone, Debug, Default)]
pub struct Filter {
    filter: Option<Regex>,
    field_info: Vec<Rc<FieldInfo>>,
}

impl Filter {
    pub fn set_regex(&mut self, filter_string: impl ToString) -> Result<&mut Self> {
        let filter_string = &filter_string.to_string();
        let re = Regex::new(&filter_string)
            .with_context(|| format!("Invalid message filter {}", filter_string))?;

        self.filter = Some(re);

        Ok(self)
    }

    pub fn add_field(&mut self, group_number: usize) {
        let new_field_info = FieldInfo::new(group_number);
        self.field_info.push(new_field_info.into());
    }

    pub fn parse(&self, text: &str) -> Result<Vec<Field>> {
        let Some(filter) = self.filter.clone() else {
            return Ok(vec![Field::raw(text)]);
        };

        let Some(captures) = filter.captures(text) else {
            bail!("Message '{}' does not match filter", text)
        };

        let mut parsed_text = Vec::default();
        for field_info in self.field_info.iter() {
            let Some(field_text) = captures.get(field_info.field_index) else {
                bail!(
                    "Message '{}' is missing match group #{}",
                    text,
                    field_info.field_index
                )
            };

            let field = Field {
                text: field_text.as_str().into(),
                field_info: field_info.clone(),
            };
            parsed_text.push(field);
        }

        Ok(parsed_text)
    }
}
