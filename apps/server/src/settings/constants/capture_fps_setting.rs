use crate::settings::NumberSetting;

pub const CAPTURE_FPS_SETTING: NumberSetting = NumberSetting {
    default: 60,
    min: 1,
    max: 240,
};
