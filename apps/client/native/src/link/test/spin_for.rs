use std::time::{Duration, Instant};

pub(crate) fn spin_for(duration: Duration) {
    let until = Instant::now() + duration;

    while Instant::now() < until {
        std::hint::spin_loop();
    }
}
