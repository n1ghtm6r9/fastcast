use std::hint::spin_loop;
use std::time::Instant;

pub fn spin_until(deadline: Instant) {
    while Instant::now() < deadline {
        spin_loop();
    }
}
