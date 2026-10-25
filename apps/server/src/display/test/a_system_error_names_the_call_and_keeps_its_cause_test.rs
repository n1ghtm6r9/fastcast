use crate::display::system_error;
use std::error::Error;
use std::io;

#[test]
fn a_system_error_names_the_call_and_keeps_its_cause() {
    let error = system_error("CreateFileW")(io::Error::from(io::ErrorKind::PermissionDenied));

    assert!(error.to_string().starts_with("CreateFileW failed: "));
    assert!(error.source().is_some());
}
