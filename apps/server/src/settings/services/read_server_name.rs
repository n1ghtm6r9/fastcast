use crate::settings::constants::{SERVER_NAME_FALLBACK_VARIABLES, SERVER_NAME_SETTING};
use crate::settings::non_blank_value;
use std::iter::once;

pub fn read_server_name(lookup: &impl Fn(&str) -> Option<String>) -> String {
    once(SERVER_NAME_SETTING.variable)
        .chain(SERVER_NAME_FALLBACK_VARIABLES)
        .find_map(|variable| non_blank_value(lookup, variable))
        .unwrap_or_else(|| SERVER_NAME_SETTING.default.to_string())
}
