pub struct RealAv1cRecord {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
    pub codec_string: &'static str,
    pub av1c: Vec<u8>,
}
