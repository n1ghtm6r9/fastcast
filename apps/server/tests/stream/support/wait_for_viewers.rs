use crate::support::{ARRIVAL_LIMIT, wait_until};
use app::hub::Hub;

pub fn wait_for_viewers(hub: &Hub, count: usize) {
    assert!(
        wait_until(ARRIVAL_LIMIT, || hub.viewer_count() == count),
        "the viewer count settles at {count}, it is {}",
        hub.viewer_count()
    );
}
