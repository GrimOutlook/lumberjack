use std::rc::Rc;

use crate::field_info::FieldInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub field_info: Rc<FieldInfo>,
    pub text: Rc<str>,
}

impl Field {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn raw(text: &str) -> Self {
        Field {
            text: text.into(),
            field_info: FieldInfo::default().into(),
        }
    }
}
