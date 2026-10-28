use super::first_output_device;
use windows::Win32::Graphics::Dxgi::IDXGIDevice;
use windows::core::Interface;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn a_device_raises_its_gpu_thread_priority() {
    let device = first_output_device();

    device.raise_priority().unwrap();

    let dxgi: IDXGIDevice = device.device().cast().unwrap();
    assert_eq!(unsafe { dxgi.GetGPUThreadPriority() }.unwrap(), 7);
}
