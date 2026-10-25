use crate::gpu::DesktopError;
use windows::Win32::Graphics::Dxgi::DXGI_ERROR_NOT_FOUND;

pub fn enumerate_all<T>(
    call: &'static str,
    enumerate: impl Fn(u32) -> windows::core::Result<T>,
) -> Result<Vec<T>, DesktopError> {
    let mut items = Vec::new();

    for index in 0.. {
        match enumerate(index) {
            Ok(item) => items.push(item),
            Err(error) if error.code() == DXGI_ERROR_NOT_FOUND => break,
            Err(source) => return Err(DesktopError::Api { call, source }),
        }
    }

    Ok(items)
}
