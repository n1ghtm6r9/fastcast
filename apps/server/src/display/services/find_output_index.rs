use crate::display::{DisplayError, output_at, output_device_name, system_error};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIFactory1};

pub fn find_output_index(device_name: &str) -> Result<Option<u32>, DisplayError> {
    let factory: IDXGIFactory1 =
        unsafe { CreateDXGIFactory1() }.map_err(system_error("CreateDXGIFactory1"))?;
    let adapter = unsafe { factory.EnumAdapters1(0) }
        .map_err(system_error("IDXGIFactory1::EnumAdapters1"))?;
    let mut index = 0;

    while let Some(output) = output_at(&adapter, index)? {
        if output_device_name(&output)?.eq_ignore_ascii_case(device_name) {
            return Ok(Some(index));
        }

        index += 1;
    }

    Ok(None)
}
