use crate::settings::NumberSetting;

pub const VIDEO_BITRATE_KBPS_SETTING: NumberSetting = NumberSetting {
    variable: "VIDEO_BITRATE_KBPS",
    default: 20000,
    min: 100,
    max: 200000,
};
