use super::first_attached_output;
use crate::gpu::GpuDevice;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn a_device_opens_on_the_adapter_of_an_output() {
    let output = first_attached_output();

    let device = GpuDevice::new(output.adapter_luid).unwrap();

    assert_eq!(device.vendor_id(), output.vendor_id);
}
