pub struct RealAvccRecord {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
    pub avcc: Vec<u8>,
}
