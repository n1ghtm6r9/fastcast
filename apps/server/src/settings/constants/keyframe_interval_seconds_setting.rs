use crate::settings::NumberSetting;

pub const KEYFRAME_INTERVAL_SECONDS_SETTING: NumberSetting = NumberSetting {
    variable: "KEYFRAME_INTERVAL_SECONDS",
    default: 2,
    min: 1,
    max: 10,
};
