use crate::packet::ByteReader;

impl<'a> ByteReader<'a> {
    pub fn take(&mut self, count: usize) -> Option<&'a [u8]> {
        let (head, tail) = self.bytes.split_at_checked(count)?;

        self.bytes = tail;

        Some(head)
    }
}
