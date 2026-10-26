use std::thread;
use std::time::Instant;

pub fn wait_until(due: Instant) {
    loop {
        let now = Instant::now();

        if now >= due {
            return;
        }

        thread::sleep(due - now);
    }
}
