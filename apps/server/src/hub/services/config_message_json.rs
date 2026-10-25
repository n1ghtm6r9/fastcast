use crate::hub::ConfigMessage;

pub fn config_message_json(message: &ConfigMessage) -> String {
    serde_json::to_string(message)
        .expect("a message made of plain strings and numbers always serializes")
}
