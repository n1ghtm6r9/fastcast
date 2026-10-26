use crate::capture::spin_until;
use std::time::{Duration, Instant};

#[test]
fn spinning_does_not_return_before_the_deadline() {
    let deadline = Instant::now() + Duration::from_millis(3);

    spin_until(deadline);

    assert!(Instant::now() >= deadline);
}
