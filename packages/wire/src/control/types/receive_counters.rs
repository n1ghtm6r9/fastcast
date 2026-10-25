#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ReceiveCounters {
    pub received_packets: u32,
    pub highest_sequence: u32,
}
