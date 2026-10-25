use crate::display::{
    AddedDisplay, DisplayError, DisplayMode, DriverHandle, VirtualDisplay, apply_mode,
    attached_display_devices, available_modes_of, bounds_of_other_displays, choose_mode,
    rightmost_edge, wait_for_new_parsec_display, wait_for_output_index,
};
use std::sync::Arc;
use tracing::info;

impl VirtualDisplay {
    pub fn open(requested: DisplayMode) -> Result<VirtualDisplay, DisplayError> {
        let driver = Arc::new(DriverHandle::open()?);
        let before = attached_display_devices();
        let added = AddedDisplay::add(driver)?;
        let device = wait_for_new_parsec_display(&before)?;
        let available = available_modes_of(&device.name);
        let mode =
            choose_mode(requested, &available).ok_or(DisplayError::NoUsableMode { requested })?;
        let left = rightmost_edge(&bounds_of_other_displays(&device.name));

        apply_mode(&device.name, mode, left, 0)?;

        let output_index = wait_for_output_index(&device.name)?;

        info!(
            "virtual display {} is ready at {mode}, DXGI output {output_index}",
            device.name
        );

        Ok(VirtualDisplay {
            output_index,
            mode,
            device_name: device.name,
            _added: added,
        })
    }
}
