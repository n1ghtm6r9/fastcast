use crate::gpu::GpuDevice;
use std::ffi::c_void;
use windows::Win32::Graphics::Direct3D11::ID3D11Query;
use windows::core::BOOL;

pub fn wait_for_gpu(device: &GpuDevice, query: &ID3D11Query) {
    let context = device.context();
    let mut finished = BOOL(0);

    unsafe { context.End(query) };

    while !finished.as_bool() {
        unsafe {
            context.GetData(
                query,
                Some(std::ptr::from_mut(&mut finished).cast::<c_void>()),
                size_of::<BOOL>() as u32,
                0,
            )
        }
        .unwrap();
    }
}
