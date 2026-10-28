use crate::gpu::DesktopError;
use windows::Win32::Foundation::E_ACCESSDENIED;
use windows::Win32::Graphics::Dxgi::{
    DXGI_ERROR_NOT_CURRENTLY_AVAILABLE, DXGI_ERROR_SESSION_DISCONNECTED,
};
use windows::core::HRESULT;

#[test]
fn a_desktop_that_comes_back_on_its_own_is_temporary() {
    let failed_with = |code: HRESULT| DesktopError::Api {
        call: "IDXGIOutput1::DuplicateOutput",
        source: windows::core::Error::from(code),
    };

    for error in [
        DesktopError::AccessLost,
        failed_with(E_ACCESSDENIED),
        failed_with(DXGI_ERROR_NOT_CURRENTLY_AVAILABLE),
        failed_with(DXGI_ERROR_SESSION_DISCONNECTED),
    ] {
        assert!(error.is_temporary(), "{error}");
    }
}
