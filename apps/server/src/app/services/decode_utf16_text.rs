pub fn decode_utf16_text(bytes: &[u8], to_unit: fn([u8; 2]) -> u16) -> Option<String> {
    let (pairs, remainder) = bytes.as_chunks::<2>();

    if !remainder.is_empty() {
        return None;
    }

    let units = pairs.iter().map(|pair| to_unit(*pair));

    char::decode_utf16(units)
        .collect::<Result<String, _>>()
        .ok()
}
