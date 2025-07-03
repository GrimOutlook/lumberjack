use std::rc::Rc;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FieldInfo {
    pub field_index: usize,
    pub custom_name: Option<Rc<str>>,
}

impl FieldInfo {
    pub fn new(field_index: usize) -> FieldInfo {
        FieldInfo {
            field_index,
            custom_name: None,
        }
    }
}
