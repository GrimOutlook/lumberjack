use std::rc::Rc;

use crate::field_info::FieldInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub field_info: Rc<FieldInfo>,
    pub text: Rc<str>,
}
