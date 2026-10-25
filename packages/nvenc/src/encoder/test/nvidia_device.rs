use super::NVIDIA_VENDOR_ID;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_UNKNOWN;
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Device,
};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIFactory1};

pub fn nvidia_device() -> ID3D11Device {
    let factory: IDXGIFactory1 = unsafe { CreateDXGIFactory1() }.unwrap();
    let adapter = (0..)
        .map_while(|index| unsafe { factory.EnumAdapters1(index) }.ok())
        .find(|adapter| {
            unsafe { adapter.GetDesc1() }
                .is_ok_and(|description| description.VendorId == NVIDIA_VENDOR_ID)
        })
        .expect("an NVIDIA adapter");
    let mut device = None;

    unsafe {
        D3D11CreateDevice(
            &adapter,
            D3D_DRIVER_TYPE_UNKNOWN,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            None,
        )
    }
    .unwrap();

    device.unwrap()
}
