use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfo {
    pub field_index: usize,
    pub name: Option<Rc<str>>,
}

impl FieldInfo {
    pub fn new(field_index: usize) -> FieldInfo {
        FieldInfo {
            field_index,
            name: None,
        }
    }
}
