use crate::settings::NumberSetting;

pub const CAPTURE_OUTPUT_SETTING: NumberSetting = NumberSetting {
    variable: "CAPTURE_OUTPUT",
    default: 0,
    min: 0,
    max: 63,
};
