use crate::settings::{NumberSetting, SettingsError, non_blank_value};

pub fn read_number_setting(
    setting: &NumberSetting,
    lookup: &impl Fn(&str) -> Option<String>,
) -> Result<u32, SettingsError> {
    let Some(raw) = non_blank_value(lookup, setting.variable) else {
        return Ok(setting.default);
    };

    match raw.parse::<u32>() {
        Ok(number) if (setting.min..=setting.max).contains(&number) => Ok(number),
        _ => Err(SettingsError {
            variable: setting.variable,
            expected: format!("a whole number from {} to {}", setting.min, setting.max),
            got: raw,
        }),
    }
}
