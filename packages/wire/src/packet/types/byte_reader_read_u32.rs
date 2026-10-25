use crate::packet::ByteReader;

impl ByteReader<'_> {
    pub fn read_u32(&mut self) -> Option<u32> {
        let bytes = self.take(4)?.try_into().ok()?;

        Some(u32::from_le_bytes(bytes))
    }
}
