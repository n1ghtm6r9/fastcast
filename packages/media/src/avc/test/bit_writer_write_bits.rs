use super::BitWriter;

impl BitWriter {
    pub fn write_bits(&mut self, value: u32, count: u32) {
        for shift in (0..count).rev() {
            self.write_bit((value >> shift) & 1 == 1);
        }
    }
}
