use crate::gpu::GpuDevice;

impl GpuDevice {
    pub fn vendor_id(&self) -> u32 {
        self.vendor_id
    }
}
