use crate::app::{ICON_ENTRY_SIZE, ICON_HEADER_SIZE, icon_entry_width};

pub fn largest_icon_picture(icon: &[u8]) -> Option<&[u8]> {
    let count = usize::from(u16::from_le_bytes(icon.get(4..6)?.try_into().ok()?));
    let entries = icon.get(ICON_HEADER_SIZE..ICON_HEADER_SIZE + count * ICON_ENTRY_SIZE)?;
    let largest = entries
        .as_chunks::<ICON_ENTRY_SIZE>()
        .0
        .iter()
        .max_by_key(|entry| icon_entry_width(entry))?;
    let size = u32::from_le_bytes(largest[8..12].try_into().ok()?) as usize;
    let offset = u32::from_le_bytes(largest[12..16].try_into().ok()?) as usize;

    icon.get(offset..offset.checked_add(size)?)
}
