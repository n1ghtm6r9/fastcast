use crate::display::{DisplayBounds, attached_display_devices, bounds_of};

pub fn bounds_of_other_displays(device_name: &str) -> Vec<DisplayBounds> {
    attached_display_devices()
        .iter()
        .filter(|device| device.name != device_name)
        .filter_map(|device| bounds_of(&device.name))
        .collect()
}
