use crate::display::{
    DisplayError, DisplaySource, DpiScaleChange, SET_DPI_SCALE, device_info_header,
    display_config_result,
};
use windows::Win32::Devices::Display::DisplayConfigSetDeviceInfo;

pub fn set_scale_level_of(source: DisplaySource, level: i32) -> Result<(), DisplayError> {
    let change = DpiScaleChange {
        header: device_info_header::<DpiScaleChange>(source, SET_DPI_SCALE),
        level,
    };

    display_config_result("DisplayConfigSetDeviceInfo", unsafe {
        DisplayConfigSetDeviceInfo((&raw const change).cast())
    })
}
