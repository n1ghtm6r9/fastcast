use crate::settings::computer_name_from;
use std::env;

pub fn computer_name() -> String {
    computer_name_from(|variable| env::var(variable).ok())
}
