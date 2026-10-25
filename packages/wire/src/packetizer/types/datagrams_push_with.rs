use crate::packetizer::Datagrams;

impl Datagrams {
    pub(crate) fn push_with(&mut self, write: impl FnOnce(&mut Vec<u8>)) {
        write(&mut self.bytes);
        self.ends.push(self.bytes.len());
    }
}
