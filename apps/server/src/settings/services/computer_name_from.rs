use crate::settings::constants::{COMPUTER_NAME_VARIABLES, UNKNOWN_COMPUTER_NAME};

pub fn computer_name_from(lookup: impl Fn(&str) -> Option<String>) -> String {
    COMPUTER_NAME_VARIABLES
        .into_iter()
        .filter_map(lookup)
        .map(|name| name.trim().to_owned())
        .find(|name| !name.is_empty())
        .unwrap_or_else(|| UNKNOWN_COMPUTER_NAME.to_owned())
}
