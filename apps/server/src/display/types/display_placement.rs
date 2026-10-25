use crate::display::DisplayBounds;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayPlacement {
    pub name: String,
    pub bounds: DisplayBounds,
    pub refresh_hz: u32,
}
