use crate::settings::SavedSettings;

pub fn named_settings(name: &str) -> SavedSettings {
    SavedSettings {
        server_name: name.to_owned(),
        ..SavedSettings::default()
    }
}
