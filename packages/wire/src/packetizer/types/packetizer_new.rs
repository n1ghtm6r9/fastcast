use crate::packetizer::Packetizer;

impl Packetizer {
    pub fn new(session: u32) -> Packetizer {
        Packetizer {
            session,
            next_sequence: 0,
            next_frame: 0,
            blocks: Vec::new(),
            plans: Vec::new(),
        }
    }
}
