use crate::window::Monitor;
use crate::window::test::{SECOND_MONITOR_PLUGGED, two_monitors};

pub fn monitors_as_plugged() -> Vec<Monitor> {
    let mut monitors = two_monitors();

    if !SECOND_MONITOR_PLUGGED.get() {
        monitors.truncate(1);
    }

    monitors
}
