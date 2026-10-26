use crate::settings::NumberSetting;

pub const VIDEO_BITRATE_KBPS_SETTING: NumberSetting = NumberSetting {
    default: 40000,
    min: 100,
    max: 200000,
};
