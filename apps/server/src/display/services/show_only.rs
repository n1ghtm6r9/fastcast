use crate::display::{
    DisplayError, DisplayMode, apply_staged_display_settings, attached_display_devices,
    detached_settings, settings_at_origin, stage_display_settings,
};
use windows::Win32::Graphics::Gdi::{CDS_SET_PRIMARY, CDS_TYPE, DISP_CHANGE};

pub fn show_only(device_name: &str, mode: DisplayMode) -> Result<(), DisplayError> {
    let refused = |code: DISP_CHANGE| DisplayError::ModeChangeRefused {
        device: device_name.to_owned(),
        mode,
        code: code.0,
    };

    stage_display_settings(device_name, &settings_at_origin(mode), CDS_SET_PRIMARY)
        .map_err(refused)?;

    for other in attached_display_devices()
        .iter()
        .filter(|device| device.name != device_name)
    {
        stage_display_settings(&other.name, &detached_settings(), CDS_TYPE(0)).map_err(|code| {
            DisplayError::DetachRefused {
                device: other.name.clone(),
                code: code.0,
            }
        })?;
    }

    apply_staged_display_settings().map_err(refused)
}
