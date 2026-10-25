use super::BitWriter;

impl BitWriter {
    pub fn write_bit(&mut self, bit: bool) {
        if self.bit_count.is_multiple_of(8) {
            self.bytes.push(0);
        }

        if bit {
            let last = self.bytes.len() - 1;

            self.bytes[last] |= 0x80 >> (self.bit_count % 8);
        }

        self.bit_count += 1;
    }
}
