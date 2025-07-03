#[derive(Debug, Default, Clone, PartialEq)]
pub enum FilterMode {
    None,
    Simple {
        fields: usize,
    },
    Regex {
        regex_string: String,
    },
    #[default]
    Automatic,
}
