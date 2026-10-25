use crate::packetizer::BlockPlan;

#[derive(Debug)]
pub struct Packetizer {
    pub(crate) session: u32,
    pub(crate) next_sequence: u32,
    pub(crate) next_frame: u32,
    pub(crate) blocks: Vec<u8>,
    pub(crate) plans: Vec<BlockPlan>,
}
