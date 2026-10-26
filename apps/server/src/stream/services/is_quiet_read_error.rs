use std::io::{Error, ErrorKind};

pub fn is_quiet_read_error(error: &Error) -> bool {
    matches!(
        error.kind(),
        ErrorKind::WouldBlock | ErrorKind::TimedOut | ErrorKind::Interrupted
    )
}
