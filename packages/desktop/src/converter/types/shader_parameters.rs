#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShaderParameters {
    pub(crate) cursor_left: i32,
    pub(crate) cursor_top: i32,
    pub(crate) cursor_width: i32,
    pub(crate) cursor_height: i32,
    pub(crate) cursor_visible: u32,
    pub(crate) cursor_kind: u32,
    pub(crate) source_width: i32,
    pub(crate) output_scale: f32,
    pub(crate) luma_black: f32,
    pub(crate) luma_range: f32,
    pub(crate) chroma_centre: f32,
    pub(crate) chroma_range: f32,
}
