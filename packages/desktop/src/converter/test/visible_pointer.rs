use crate::cursor::CursorImage;
use crate::duplication::PointerState;

pub fn visible_pointer(shape: CursorImage, left: i32, top: i32, generation: u64) -> PointerState {
    PointerState {
        visible: true,
        left,
        top,
        shape: Some(shape),
        shape_generation: generation,
    }
}
