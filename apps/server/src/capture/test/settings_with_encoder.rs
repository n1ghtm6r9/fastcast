use crate::settings::{Settings, settings_from};

pub fn settings_with_encoder(name: &'static str) -> Settings {
    settings_from(|key| (key == "VIDEO_ENCODER").then(|| name.to_owned()))
        .expect("the encoder name is valid")
}
