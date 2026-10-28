use crate::gpu::DesktopError;
use windows::Win32::Foundation::{E_FAIL, E_INVALIDARG};
use windows::Win32::Graphics::Dxgi::{
    DXGI_ERROR_DEVICE_REMOVED, DXGI_ERROR_NOT_FOUND, DXGI_ERROR_UNSUPPORTED,
};
use windows::core::HRESULT;

#[test]
fn a_desktop_error_that_needs_a_new_device_is_not_temporary() {
    let failed_with = |code: HRESULT| DesktopError::Api {
        call: "IDXGIOutput1::DuplicateOutput",
        source: windows::core::Error::from(code),
    };

    for error in [
        failed_with(E_FAIL),
        failed_with(E_INVALIDARG),
        failed_with(DXGI_ERROR_UNSUPPORTED),
        failed_with(DXGI_ERROR_DEVICE_REMOVED),
        failed_with(DXGI_ERROR_NOT_FOUND),
        DesktopError::AdapterNotFound,
    ] {
        assert!(!error.is_temporary(), "{error}");
    }
}
