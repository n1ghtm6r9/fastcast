use crate::display::{DisplayDevice, display_device_at, text_of_wide};
use windows::Win32::Graphics::Gdi::DISPLAY_DEVICE_ATTACHED_TO_DESKTOP;

pub fn attached_display_devices() -> Vec<DisplayDevice> {
    (0..)
        .map_while(display_device_at)
        .filter(|device| {
            device
                .StateFlags
                .contains(DISPLAY_DEVICE_ATTACHED_TO_DESKTOP)
        })
        .map(|device| DisplayDevice {
            name: text_of_wide(&device.DeviceName),
            adapter: text_of_wide(&device.DeviceString),
        })
        .collect()
}
