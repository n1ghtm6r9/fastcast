use crate::avc::BitReader;

impl BitReader<'_> {
    pub fn read_bit(&mut self) -> Option<u32> {
        let byte = *self.data.get(self.position / 8)?;
        let bit = (byte >> (7 - self.position % 8)) & 1;

        self.position += 1;

        Some(u32::from(bit))
    }
}
