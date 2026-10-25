use crate::gpu::DesktopError;
use std::error::Error;

#[test]
fn a_plain_desktop_error_has_no_source() {
    let errors = [
        DesktopError::AccessLost,
        DesktopError::AdapterNotFound,
        DesktopError::OddSize {
            width: 63,
            height: 32,
        },
        DesktopError::Rotated,
        DesktopError::ShaderCompile(String::from("X3000")),
    ];

    assert!(errors.iter().all(|error| error.source().is_none()));
}
