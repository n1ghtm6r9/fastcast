use std::thread;
use std::time::{Duration, Instant};

pub fn poll_until<T, E>(
    timeout: Duration,
    interval: Duration,
    mut attempt: impl FnMut() -> Result<Option<T>, E>,
) -> Result<Option<T>, E> {
    let deadline = Instant::now() + timeout;

    loop {
        if let Some(found) = attempt()? {
            return Ok(Some(found));
        }

        if Instant::now() >= deadline {
            return Ok(None);
        }

        thread::sleep(interval);
    }
}
