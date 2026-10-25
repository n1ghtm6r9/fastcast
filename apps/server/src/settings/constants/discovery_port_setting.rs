use crate::settings::NumberSetting;

pub const DISCOVERY_PORT_SETTING: NumberSetting = NumberSetting {
    variable: "DISCOVERY_PORT",
    default: 3278,
    min: 1,
    max: 65535,
};
