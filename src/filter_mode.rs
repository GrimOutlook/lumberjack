#[derive(Debug, Default, Clone, PartialEq)]
pub enum FilterMode {
    Simple {
        fields: usize,
    },
    Regex {
        regex_string: String,
    },
    #[default]
    Automatic,
}

