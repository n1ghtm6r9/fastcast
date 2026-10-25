use crate::display::{DisplayError, system_error};
use windows::Win32::Graphics::Dxgi::{DXGI_ERROR_NOT_FOUND, IDXGIAdapter1, IDXGIOutput};

pub fn output_at(adapter: &IDXGIAdapter1, index: u32) -> Result<Option<IDXGIOutput>, DisplayError> {
    match unsafe { adapter.EnumOutputs(index) } {
        Ok(output) => Ok(Some(output)),
        Err(error) if error.code() == DXGI_ERROR_NOT_FOUND => Ok(None),
        Err(error) => Err(system_error("IDXGIAdapter::EnumOutputs")(error)),
    }
}
