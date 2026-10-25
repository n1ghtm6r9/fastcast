use crate::cursor::CursorImage;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PointerState {
    pub visible: bool,
    pub left: i32,
    pub top: i32,
    pub shape: Option<CursorImage>,
    pub shape_generation: u64,
}
