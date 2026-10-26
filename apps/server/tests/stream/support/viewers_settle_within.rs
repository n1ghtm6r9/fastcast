use crate::support::wait_until;
use app::hub::Hub;
use std::time::Duration;

pub fn viewers_settle_within(hub: &Hub, count: usize, limit: Duration) -> bool {
    wait_until(limit, || hub.viewer_count() == count)
}
