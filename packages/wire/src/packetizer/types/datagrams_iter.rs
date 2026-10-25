use crate::packetizer::Datagrams;

impl Datagrams {
    pub fn iter(&self) -> impl Iterator<Item = &[u8]> + '_ {
        (0..self.len()).map(|index| self.get(index))
    }
}
