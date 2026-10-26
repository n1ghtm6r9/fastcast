use crate::app::ICON_ENTRY_SIZE;

pub fn icon_entry_width(entry: &[u8; ICON_ENTRY_SIZE]) -> u32 {
    match entry[0] {
        0 => 256,
        width => u32::from(width),
    }
}
