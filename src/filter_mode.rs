pub struct SimpleFilter {
    split_character: char,
    fields: usize,
}

pub struct RegexFilter {
    regex_string: String,
}

/// Used to parse messages automatically with no user input.
///
/// Will likely be less accurate than a specifically made regex filter.
pub struct AutomaticFilter {}

/// Used to parse messages that are specifically in a json format.
pub struct JsonFilter {}
