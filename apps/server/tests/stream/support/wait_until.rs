use crate::support::POLL_STEP;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn wait_until(limit: Duration, mut condition: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + limit;

    loop {
        if condition() {
            return true;
        }

        if Instant::now() >= deadline {
            return false;
        }

        sleep(POLL_STEP);
    }
}
