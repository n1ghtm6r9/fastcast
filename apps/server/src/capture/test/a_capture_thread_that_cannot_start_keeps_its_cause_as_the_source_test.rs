use crate::capture::NativeError;
use std::error::Error;
use std::io;

#[test]
fn a_capture_thread_that_cannot_start_keeps_its_cause_as_the_source() {
    let error = NativeError::Spawn(io::Error::other("no threads left"));

    let source = error.source().expect("the spawn error is the source");

    assert_eq!(source.to_string(), "no threads left");
}
