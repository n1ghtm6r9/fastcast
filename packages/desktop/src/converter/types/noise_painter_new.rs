use crate::converter::{
    NOISE_SHADER_SOURCE, NoisePainter, NoiseParameters, create_constant_buffer, create_pixel_shader,
};
use crate::gpu::{DesktopError, GpuDevice};

impl NoisePainter {
    pub fn new(device: &GpuDevice) -> Result<NoisePainter, DesktopError> {
        Ok(NoisePainter {
            shader: create_pixel_shader(device, NOISE_SHADER_SOURCE)?,
            constants: create_constant_buffer::<NoiseParameters>(device)?,
        })
    }
}
