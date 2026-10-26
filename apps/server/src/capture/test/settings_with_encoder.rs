use crate::settings::{SavedSettings, Settings, encoder_choice_named, settings_of};

pub fn settings_with_encoder(name: &'static str) -> Settings {
    let saved = SavedSettings {
        video_encoder: encoder_choice_named(name).expect("the encoder name is valid"),
        ..SavedSettings::default()
    };

    settings_of(&saved, "fastcast")
}
