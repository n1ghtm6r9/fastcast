use crate::display::DisplayError;
use std::io;

pub fn display_config_result(call: &'static str, status: i32) -> Result<(), DisplayError> {
    match status {
        0 => Ok(()),
        code => Err(DisplayError::System {
            call,
            source: io::Error::from_raw_os_error(code),
        }),
    }
}
