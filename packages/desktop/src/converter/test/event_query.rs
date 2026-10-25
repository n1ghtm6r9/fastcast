use crate::gpu::GpuDevice;
use windows::Win32::Graphics::Direct3D11::{D3D11_QUERY_DESC, D3D11_QUERY_EVENT, ID3D11Query};

pub fn event_query(device: &GpuDevice) -> ID3D11Query {
    let description = D3D11_QUERY_DESC {
        Query: D3D11_QUERY_EVENT,
        MiscFlags: 0,
    };
    let mut query = None;

    unsafe { device.device().CreateQuery(&description, Some(&mut query)) }.unwrap();

    query.unwrap()
}
