#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoiseParameters {
    pub(crate) frame: u32,
    pub(crate) levels: u32,
    pub(crate) output_scale: f32,
    pub(crate) padding: u32,
}
