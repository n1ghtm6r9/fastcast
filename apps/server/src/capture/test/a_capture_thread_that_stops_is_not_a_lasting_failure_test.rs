use crate::capture::NativeError;
use std::io;

#[test]
fn a_capture_thread_that_stops_is_not_a_lasting_failure() {
    assert!(!NativeError::WorkerGone.is_lasting());
    assert!(!NativeError::Spawn(io::Error::other("no threads left")).is_lasting());
}
