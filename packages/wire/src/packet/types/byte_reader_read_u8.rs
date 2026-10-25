use crate::packet::ByteReader;

impl ByteReader<'_> {
    pub fn read_u8(&mut self) -> Option<u8> {
        self.take(1)?.first().copied()
    }
}
