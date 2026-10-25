use crate::packetizer::Datagrams;

impl Datagrams {
    pub fn get(&self, index: usize) -> &[u8] {
        let start = if index == 0 { 0 } else { self.ends[index - 1] };

        &self.bytes[start..self.ends[index]]
    }
}
