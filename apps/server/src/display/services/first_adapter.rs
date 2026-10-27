use crate::display::{DisplayError, system_error};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory1};

pub fn first_adapter() -> Result<IDXGIAdapter1, DisplayError> {
    let factory: IDXGIFactory1 =
        unsafe { CreateDXGIFactory1() }.map_err(system_error("CreateDXGIFactory1"))?;

    unsafe { factory.EnumAdapters1(0) }.map_err(system_error("IDXGIFactory1::EnumAdapters1"))
}
