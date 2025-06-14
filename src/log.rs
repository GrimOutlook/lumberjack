use crate::{field::Field, field_info::FieldInfo, log_message::LogMessage};

#[derive(Debug, Clone, PartialEq)]
pub struct Log {
    pub messages: Vec<LogMessage>,
}

// impl FromIterator<LogMessage> for Log {
//     fn from_iter<I: IntoIterator<Item = LogMessage>>(iter: I) -> Self {
//         let fields = iter
//             .into_iter()
//             .enumerate()
//             .map(| text| LogMessage {
//
//             })
//             .collect();
//
//         Log { fields }
//     }
// }
