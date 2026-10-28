use crate::colour::BitDepth;
use crate::converter::{
    CHROMA_SHADER_SOURCE, Converter, LUMA_SHADER_SOURCE, ShaderParameters, chroma_format,
    create_constant_buffer, create_output_texture, create_pixel_shader, create_plane_target,
    create_rasterizer_state, create_vertex_shader, luma_format,
};
use crate::gpu::{DesktopError, GpuDevice, require_even_size};

impl Converter {
    pub fn new(
        device: &GpuDevice,
        width: u32,
        height: u32,
        depth: BitDepth,
    ) -> Result<Converter, DesktopError> {
        require_even_size(width, height)?;

        let output = create_output_texture(device, width, height, depth)?;

        Ok(Converter {
            depth,
            width,
            height,
            vertex_shader: create_vertex_shader(device)?,
            luma_shader: create_pixel_shader(device, LUMA_SHADER_SOURCE)?,
            chroma_shader: create_pixel_shader(device, CHROMA_SHADER_SOURCE)?,
            rasterizer: create_rasterizer_state(device)?,
            constants: create_constant_buffer::<ShaderParameters>(device)?,
            luma_target: create_plane_target(device, &output, luma_format(depth))?,
            chroma_target: create_plane_target(device, &output, chroma_format(depth))?,
            output,
            cursor: None,
        })
    }
}
