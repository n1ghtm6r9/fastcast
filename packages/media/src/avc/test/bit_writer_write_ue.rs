use super::BitWriter;

impl BitWriter {
    pub fn write_ue(&mut self, value: u32) {
        let code_number = u64::from(value) + 1;
        let length = 64 - code_number.leading_zeros();

        for _ in 1..length {
            self.write_bit(false);
        }

        for shift in (0..length).rev() {
            self.write_bit((code_number >> shift) & 1 == 1);
        }
    }
}
