use super::BitWriter;

impl BitWriter {
    pub fn finish(mut self) -> Vec<u8> {
        self.write_bit(true);

        self.bytes
    }
}
