use crate::packetizer::Datagrams;

impl Datagrams {
    pub fn clear(&mut self) {
        self.bytes.clear();
        self.ends.clear();
    }
}
