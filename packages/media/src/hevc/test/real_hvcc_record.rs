pub struct RealHvccRecord {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
    pub codec_string: &'static str,
    pub hvcc: Vec<u8>,
}
