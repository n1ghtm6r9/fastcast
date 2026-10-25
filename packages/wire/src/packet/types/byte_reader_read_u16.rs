use crate::packet::ByteReader;

impl ByteReader<'_> {
    pub fn read_u16(&mut self) -> Option<u16> {
        let bytes = self.take(2)?.try_into().ok()?;

        Some(u16::from_le_bytes(bytes))
    }
}
