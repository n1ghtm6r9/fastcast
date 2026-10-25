use crate::packetizer::Datagrams;

impl Datagrams {
    pub fn is_empty(&self) -> bool {
        self.ends.is_empty()
    }
}
