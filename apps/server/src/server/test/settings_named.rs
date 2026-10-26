use crate::settings::{SavedSettings, Settings, settings_of};

pub fn settings_named(name: &str) -> Settings {
    settings_of(
        &SavedSettings {
            server_name: name.to_owned(),
            ..SavedSettings::default()
        },
        "fastcast",
    )
}
