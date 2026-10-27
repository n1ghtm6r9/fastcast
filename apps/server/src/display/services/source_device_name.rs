use crate::display::{
    DisplayError, DisplaySource, device_info_header, display_config_result, text_of_wide,
};
use windows::Win32::Devices::Display::{
    DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME, DISPLAYCONFIG_SOURCE_DEVICE_NAME,
    DisplayConfigGetDeviceInfo,
};

pub fn source_device_name(source: DisplaySource) -> Result<String, DisplayError> {
    let mut name = DISPLAYCONFIG_SOURCE_DEVICE_NAME {
        header: device_info_header::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>(
            source,
            DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
        ),
        ..Default::default()
    };

    display_config_result("DisplayConfigGetDeviceInfo", unsafe {
        DisplayConfigGetDeviceInfo((&raw mut name).cast())
    })?;

    Ok(text_of_wide(&name.viewGdiDeviceName))
}
