use crate::settings::NumberSetting;

pub const VIDEO_BITRATE_MBPS_SETTING: NumberSetting = NumberSetting {
    default: 60,
    min: 1,
    max: 200,
};
