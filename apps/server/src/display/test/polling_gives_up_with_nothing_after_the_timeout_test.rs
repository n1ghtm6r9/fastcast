use crate::display::poll_until;
use std::convert::Infallible;
use std::time::{Duration, Instant};

#[test]
fn polling_gives_up_with_nothing_after_the_timeout() {
    let mut attempts = 0;
    let started = Instant::now();

    let found = poll_until(Duration::from_millis(30), Duration::from_millis(5), || {
        attempts += 1;

        Ok::<Option<u32>, Infallible>(None)
    });

    assert_eq!(found, Ok(None));
    assert!(attempts > 1);
    assert!(started.elapsed() >= Duration::from_millis(30));
}
