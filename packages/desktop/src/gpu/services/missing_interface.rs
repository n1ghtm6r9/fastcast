use crate::gpu::DesktopError;
use windows::Win32::Foundation::E_POINTER;

pub fn missing_interface(call: &'static str) -> DesktopError {
    DesktopError::Api {
        call,
        source: windows::core::Error::from(E_POINTER),
    }
}
