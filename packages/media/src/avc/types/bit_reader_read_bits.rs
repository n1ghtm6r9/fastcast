use crate::avc::BitReader;

impl BitReader<'_> {
    pub fn read_bits(&mut self, count: u32) -> Option<u32> {
        if count > 32 {
            return None;
        }

        let mut value: u64 = 0;

        for _ in 0..count {
            value = (value << 1) | u64::from(self.read_bit()?);
        }

        u32::try_from(value).ok()
    }
}
