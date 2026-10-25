use crate::packetizer::Datagrams;

impl Datagrams {
    pub fn len(&self) -> usize {
        self.ends.len()
    }
}
