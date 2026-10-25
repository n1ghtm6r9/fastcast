use crate::settings::{TextSetting, non_blank_value};

pub fn read_text_setting(
    setting: &TextSetting,
    lookup: &impl Fn(&str) -> Option<String>,
) -> String {
    non_blank_value(lookup, setting.variable).unwrap_or_else(|| setting.default.to_string())
}
