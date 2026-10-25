use crate::display::DisplayDevice;

pub fn device_named(name: &str, adapter: &str) -> DisplayDevice {
    DisplayDevice {
        name: name.to_owned(),
        adapter: adapter.to_owned(),
    }
}
