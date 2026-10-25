use crate::display::{
    DisplayPlacement, attached_display_devices, current_settings_of, placement_from,
};

pub fn current_placements() -> Vec<DisplayPlacement> {
    attached_display_devices()
        .iter()
        .filter_map(|device| {
            current_settings_of(&device.name)
                .map(|settings| placement_from(&device.name, &settings))
        })
        .collect()
}
