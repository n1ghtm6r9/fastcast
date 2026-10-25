use crate::cursor::CursorKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CursorImage {
    pub width: u32,
    pub height: u32,
    pub kind: CursorKind,
    pub pixels: Vec<u8>,
}
