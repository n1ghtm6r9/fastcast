use crate::settings::NumberSetting;

pub const KEYFRAME_INTERVAL_SECONDS_SETTING: NumberSetting = NumberSetting {
    default: 1,
    min: 1,
    max: 10,
};
