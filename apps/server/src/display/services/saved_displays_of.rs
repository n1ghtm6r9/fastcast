use crate::display::{DisplayDevice, SavedDisplay, current_settings_of, placement_from};
use windows::Win32::Graphics::Gdi::{
    DEVMODEW, DM_DISPLAYFREQUENCY, DM_PELSHEIGHT, DM_PELSWIDTH, DM_POSITION,
};

pub fn saved_displays_of(devices: &[DisplayDevice]) -> Vec<SavedDisplay> {
    devices
        .iter()
        .filter_map(|device| {
            let settings = current_settings_of(&device.name)?;

            Some(SavedDisplay {
                placement: placement_from(&device.name, &settings),
                settings: DEVMODEW {
                    dmFields: settings.dmFields
                        | DM_POSITION
                        | DM_PELSWIDTH
                        | DM_PELSHEIGHT
                        | DM_DISPLAYFREQUENCY,
                    ..settings
                },
            })
        })
        .collect()
}
