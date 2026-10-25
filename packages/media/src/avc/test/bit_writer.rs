#[derive(Default)]
pub struct BitWriter {
    pub(super) bytes: Vec<u8>,
    pub(super) bit_count: usize,
}
