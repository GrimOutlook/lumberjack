use std::rc::Rc;

use anyhow::{Context, Result, bail};
use regex::Regex;

use crate::{field::Field, field_info::FieldInfo};

#[derive(Clone, Debug)]
pub struct Filter {
    filter: Regex,
    field_info: Vec<Rc<FieldInfo>>,
}

impl Filter {
    pub fn new(filter_string: impl ToString) -> Result<Filter> {
        let filter_string = &filter_string.to_string();
        let re = Regex::new(&filter_string)
            .with_context(|| format!("Invalid message filter {}", filter_string))?;

        let filter = Filter {
            filter: re,
            field_info: Vec::default(),
        };

        Ok(filter)
    }

    pub fn add_field(&mut self, group_number: usize) {
        let new_field_info = FieldInfo::new(group_number);
        self.field_info.push(new_field_info.into());
    }

    pub fn parse(&self, text: &str) -> Result<Vec<Field>> {
        let Some(captures) = self.filter.captures(text) else {
            bail!("Message '{}' does not match filter", text)
        };

        let mut parsed_text = Vec::default();
        for field_info in self.field_info.iter() {
            let Some(field_text) = captures.get(field_info.group_number) else {
                bail!(
                    "Message '{}' is missing match group #{}",
                    text,
                    field_info.group_number
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
