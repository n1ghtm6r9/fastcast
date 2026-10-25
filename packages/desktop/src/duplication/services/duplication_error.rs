use crate::gpu::DesktopError;
use windows::Win32::Graphics::Dxgi::DXGI_ERROR_ACCESS_LOST;

pub fn duplication_error(call: &'static str, error: windows::core::Error) -> DesktopError {
    if error.code() == DXGI_ERROR_ACCESS_LOST {
        DesktopError::AccessLost
    } else {
        DesktopError::Api {
            call,
            source: error,
        }
    }
}
