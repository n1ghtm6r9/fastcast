use crate::capture::NativeError;
use std::io;

#[test]
fn a_capture_thread_that_cannot_start_says_why() {
    let error = NativeError::Spawn(io::Error::other("no threads left"));

    assert_eq!(
        error.to_string(),
        "could not start the capture thread: no threads left"
    );
}
