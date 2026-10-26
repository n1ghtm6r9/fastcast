use crate::link::is_quiet_read_error;
use std::io::{Error, ErrorKind};

#[test]
fn a_read_that_only_ran_out_of_time_does_not_end_the_link() {
    assert!(is_quiet_read_error(&Error::from(ErrorKind::WouldBlock)));
    assert!(is_quiet_read_error(&Error::from(ErrorKind::TimedOut)));
    assert!(is_quiet_read_error(&Error::from(ErrorKind::Interrupted)));
    assert!(!is_quiet_read_error(&Error::from(
        ErrorKind::ConnectionReset
    )));
    assert!(!is_quiet_read_error(&Error::from(
        ErrorKind::PermissionDenied
    )));
}
