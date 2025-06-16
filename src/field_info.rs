use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfo {
    pub group_number: usize,
    pub custom_name: Option<Rc<str>>,
}

impl FieldInfo {
    pub fn new(field_index: usize) -> FieldInfo {
        FieldInfo {
            group_number: field_index,
            custom_name: None,
        }
    }
}
