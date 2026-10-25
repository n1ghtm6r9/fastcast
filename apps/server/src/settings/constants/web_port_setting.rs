use crate::settings::NumberSetting;

pub const WEB_PORT_SETTING: NumberSetting = NumberSetting {
    variable: "WEB_PORT",
    default: 3278,
    min: 1,
    max: 65535,
};
