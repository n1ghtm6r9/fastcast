use crate::display::{
    DisplayError, DisplaySource, DpiScaleRequest, GET_DPI_SCALE, ScaleLevels, device_info_header,
    display_config_result,
};
use windows::Win32::Devices::Display::DisplayConfigGetDeviceInfo;

pub fn scale_levels_of(source: DisplaySource) -> Result<ScaleLevels, DisplayError> {
    let mut request = DpiScaleRequest {
        header: device_info_header::<DpiScaleRequest>(source, GET_DPI_SCALE),
        lowest: 0,
        current: 0,
        highest: 0,
    };

    display_config_result("DisplayConfigGetDeviceInfo", unsafe {
        DisplayConfigGetDeviceInfo((&raw mut request).cast())
    })?;

    Ok(ScaleLevels {
        lowest: request.lowest,
        current: request.current,
        highest: request.highest,
    })
}
