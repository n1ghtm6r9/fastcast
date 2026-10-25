use crate::duplication::{
    Duplication, PointerState, create_duplication, create_frame_texture, find_output,
    require_upright,
};
use crate::gpu::{DesktopError, GpuDevice, OutputInfo, require_even_size};

impl Duplication {
    pub fn new(device: &GpuDevice, output: &OutputInfo) -> Result<Duplication, DesktopError> {
        let dxgi_output = find_output(device, &output.device_name)?;
        let duplication = create_duplication(device, &dxgi_output)?;
        let description = unsafe { duplication.GetDesc() };
        let width = description.ModeDesc.Width;
        let height = description.ModeDesc.Height;

        require_upright(description.Rotation)?;
        require_even_size(width, height)?;

        Ok(Duplication {
            duplication,
            frame: create_frame_texture(device, width, height)?,
            pointer: PointerState::default(),
            width,
            height,
        })
    }
}
