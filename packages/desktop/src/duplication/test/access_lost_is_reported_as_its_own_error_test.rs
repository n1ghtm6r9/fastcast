use crate::duplication::duplication_error;
use crate::gpu::DesktopError;
use windows::Win32::Graphics::Dxgi::DXGI_ERROR_ACCESS_LOST;

#[test]
fn access_lost_is_reported_as_its_own_error() {
    let error = duplication_error("x", DXGI_ERROR_ACCESS_LOST.into());

    assert!(matches!(error, DesktopError::AccessLost));
}
