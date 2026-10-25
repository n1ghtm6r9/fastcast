use crate::gpu::DesktopError;
use windows::Win32::Foundation::E_FAIL;

#[test]
fn desktop_errors_name_the_failed_call() {
    let error = DesktopError::Api {
        call: "D3D11CreateDevice",
        source: windows::core::Error::from(E_FAIL),
    };

    assert!(error.to_string().contains("D3D11CreateDevice failed"));
}
