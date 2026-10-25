pub fn u16_at(bytes: &[u8], position: usize) -> Option<u16> {
    let pair = bytes.get(position..position.checked_add(2)?)?;

    Some(u16::from_be_bytes([pair[0], pair[1]]))
}
