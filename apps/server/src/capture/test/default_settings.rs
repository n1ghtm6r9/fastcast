use crate::settings::{Settings, settings_from};

pub fn default_settings() -> Settings {
    settings_from(|_| None).expect("the defaults are valid")
}
