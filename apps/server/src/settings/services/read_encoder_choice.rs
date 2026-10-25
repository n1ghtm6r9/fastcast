use crate::settings::{
    EncoderChoice, SettingsError, TextSetting, encoder_choice_named, read_text_setting,
};

pub fn read_encoder_choice(
    setting: &TextSetting,
    lookup: &impl Fn(&str) -> Option<String>,
) -> Result<EncoderChoice, SettingsError> {
    let name = read_text_setting(setting, lookup);

    encoder_choice_named(&name).ok_or_else(|| SettingsError {
        variable: setting.variable,
        expected: "one of auto, nvenc, amf, qsv, x264".to_string(),
        got: name,
    })
}
