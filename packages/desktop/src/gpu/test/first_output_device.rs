use super::first_attached_output;
use crate::gpu::GpuDevice;

pub fn first_output_device() -> GpuDevice {
    GpuDevice::new(first_attached_output().adapter_luid).unwrap()
}
