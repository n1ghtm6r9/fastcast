use crate::settings::{
    BackendChoice, SettingsError, TextSetting, backend_choice_named, read_text_setting,
};

pub fn read_backend_choice(
    setting: &TextSetting,
    lookup: &impl Fn(&str) -> Option<String>,
) -> Result<BackendChoice, SettingsError> {
    let name = read_text_setting(setting, lookup);

    backend_choice_named(&name).ok_or_else(|| SettingsError {
        variable: setting.variable,
        expected: "one of auto, native, ffmpeg".to_string(),
        got: name,
    })
}
