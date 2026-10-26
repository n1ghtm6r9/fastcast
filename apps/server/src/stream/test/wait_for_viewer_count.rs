use crate::hub::Hub;
use std::thread;
use std::time::{Duration, Instant};

pub fn wait_for_viewer_count(hub: &Hub, expected: usize) -> bool {
    let deadline = Instant::now() + Duration::from_secs(2);

    while hub.viewer_count() != expected && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(5));
    }

    hub.viewer_count() == expected
}
