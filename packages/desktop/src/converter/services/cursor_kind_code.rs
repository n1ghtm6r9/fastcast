use crate::converter::{CURSOR_KIND_ALPHA, CURSOR_KIND_MASKED};
use crate::cursor::CursorKind;

pub fn cursor_kind_code(kind: CursorKind) -> u32 {
    match kind {
        CursorKind::Alpha => CURSOR_KIND_ALPHA,
        CursorKind::Masked => CURSOR_KIND_MASKED,
    }
}
