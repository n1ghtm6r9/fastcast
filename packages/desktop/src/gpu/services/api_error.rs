use crate::gpu::DesktopError;

pub fn api_error(call: &'static str) -> impl FnOnce(windows::core::Error) -> DesktopError {
    move |source| DesktopError::Api { call, source }
}
