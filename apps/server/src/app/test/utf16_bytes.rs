pub fn utf16_bytes(text: &str, to_bytes: fn(u16) -> [u8; 2]) -> Vec<u8> {
    text.encode_utf16().flat_map(to_bytes).collect()
}
