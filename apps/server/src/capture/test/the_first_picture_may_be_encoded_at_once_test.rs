use crate::capture::FrameClock;
use std::time::Instant;

#[test]
fn the_first_picture_may_be_encoded_at_once() {
    assert!(FrameClock::new(100).may_encode(Instant::now()));
}
