use crate::gpu::{DesktopError, api_error, enumerate_all};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory1};

pub fn list_adapters() -> Result<Vec<IDXGIAdapter1>, DesktopError> {
    let factory: IDXGIFactory1 =
        unsafe { CreateDXGIFactory1() }.map_err(api_error("CreateDXGIFactory1"))?;

    enumerate_all("IDXGIFactory1::EnumAdapters1", |index| unsafe {
        factory.EnumAdapters1(index)
    })
}
