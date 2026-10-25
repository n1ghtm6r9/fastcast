use crate::display::{
    AddedDisplay, DesktopTakeover, DisplayError, DisplayMode, DriverHandle, VirtualDisplay,
    attached_display_devices, available_modes_of, choose_mode, saved_displays_of, show_only,
    wait_for_new_parsec_display, wait_for_output_index,
};
use std::sync::Arc;
use tracing::info;

impl VirtualDisplay {
    pub fn open(requested: DisplayMode) -> Result<VirtualDisplay, DisplayError> {
        let driver = Arc::new(DriverHandle::open()?);
        let before = attached_display_devices();
        let saved = saved_displays_of(&before);
        let takeover = DesktopTakeover {
            added: Some(AddedDisplay::add(driver)?),
            saved,
        };
        let device = wait_for_new_parsec_display(&before)?;
        let available = available_modes_of(&device.name);
        let mode =
            choose_mode(requested, &available).ok_or(DisplayError::NoUsableMode { requested })?;

        show_only(&device.name, mode)?;

        let output_index = wait_for_output_index(&device.name)?;

        info!(
            "virtual display {} at {mode} is the only screen now (DXGI output {output_index}), the monitors stay off while the TV watches",
            device.name
        );

        Ok(VirtualDisplay {
            output_index,
            mode,
            device_name: device.name,
            _takeover: takeover,
        })
    }
}
