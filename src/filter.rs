use std::{collections::HashMap, rc::Rc};

use crate::field_info::FieldInfo;

#[derive(Clone, Debug)]
pub struct Filter {
    filter: Regext,
    field_info: Vec<Rc<FieldInfo>>,
}

impl Filter {
    pub fn new(filter_string: impl ToString) -> Filter {
        Filter {
            filter: filter_string.to_string().into(),
            field_info: Vec::default(),
        }
    }

    pub fn add_field(&mut self, group_number: usize) {
        let new_field_info = FieldInfo::new(group_number);
        self.field_info.push(new_field_info.into());
    }
}
