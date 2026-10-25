use crate::avc::BitReader;

const MOST_LEADING_ZEROS: u32 = 31;

impl BitReader<'_> {
    pub fn read_ue(&mut self) -> Option<u32> {
        let mut leading_zeros = 0;

        while self.read_bit()? == 0 {
            leading_zeros += 1;

            if leading_zeros > MOST_LEADING_ZEROS {
                return None;
            }
        }

        let suffix = u64::from(self.read_bits(leading_zeros)?);
        let value = (1u64 << leading_zeros) - 1 + suffix;

        u32::try_from(value).ok()
    }
}
