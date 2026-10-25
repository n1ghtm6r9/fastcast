use crate::settings::{SettingsError, TextSetting, read_text_setting, switch_named};

pub fn read_switch_setting(
    setting: &TextSetting,
    lookup: &impl Fn(&str) -> Option<String>,
) -> Result<bool, SettingsError> {
    let name = read_text_setting(setting, lookup);

    switch_named(&name).ok_or_else(|| SettingsError {
        variable: setting.variable,
        expected: "on or off".to_string(),
        got: name,
    })
}
