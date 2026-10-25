use crate::packet::ByteReader;
use std::mem;

impl<'a> ByteReader<'a> {
    pub fn rest(&mut self) -> &'a [u8] {
        mem::take(&mut self.bytes)
    }
}
