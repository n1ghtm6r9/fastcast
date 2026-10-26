#[cfg(windows)]
use crate::capture::{FIRST_ADAPTER_INDEX, HARDWARE_DEFAULT_REFRESH_HZ};
#[cfg(windows)]
use crate::display::current_settings_of;

#[cfg(windows)]
pub fn monitor_refresh_hz(output: u32) -> Option<u32> {
    let outputs = desktop::list_outputs().ok()?;
    let monitor = outputs
        .iter()
        .find(|info| info.adapter_index == FIRST_ADAPTER_INDEX && info.output_index == output)?;

    current_settings_of(&monitor.device_name)
        .map(|mode| mode.dmDisplayFrequency)
        .filter(|&refresh_hz| refresh_hz > HARDWARE_DEFAULT_REFRESH_HZ)
}

#[cfg(not(windows))]
pub fn monitor_refresh_hz(_output: u32) -> Option<u32> {
    None
}
