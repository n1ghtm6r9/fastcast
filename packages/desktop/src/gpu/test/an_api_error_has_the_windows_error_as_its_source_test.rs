use crate::gpu::DesktopError;
use std::error::Error;
use windows::Win32::Foundation::E_FAIL;

#[test]
fn an_api_error_has_the_windows_error_as_its_source() {
    let error = DesktopError::Api {
        call: "D3D11CreateDevice",
        source: windows::core::Error::from(E_FAIL),
    };

    let source = error.source().unwrap();

    assert_eq!(
        source.to_string(),
        windows::core::Error::from(E_FAIL).to_string()
    );
}
