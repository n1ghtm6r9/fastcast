use crate::colour::BitDepth;
use crate::converter::{
    CURSOR_KIND_ALPHA, ShaderParameters, UploadedCursor, cursor_kind_code, output_scale,
};
use crate::duplication::PointerState;

impl ShaderParameters {
    pub fn new(
        depth: BitDepth,
        source_width: u32,
        pointer: &PointerState,
        cursor: Option<&UploadedCursor>,
    ) -> ShaderParameters {
        let drawn = cursor.filter(|_| pointer.visible);

        ShaderParameters {
            cursor_left: pointer.left,
            cursor_top: pointer.top,
            cursor_width: drawn.map_or(0, |cursor| cursor.width),
            cursor_height: drawn.map_or(0, |cursor| cursor.height),
            cursor_visible: u32::from(drawn.is_some()),
            cursor_kind: drawn.map_or(CURSOR_KIND_ALPHA, |cursor| cursor_kind_code(cursor.kind)),
            source_width: source_width as i32,
            output_scale: output_scale(depth),
            luma_black: depth.luma_black(),
            luma_range: depth.luma_range(),
            chroma_centre: depth.chroma_centre(),
            chroma_range: depth.chroma_range(),
        }
    }
}
