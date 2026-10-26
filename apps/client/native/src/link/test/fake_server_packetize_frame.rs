use crate::link::test::{FakeServer, packetized_frame};
use wire::FrameKind;

impl FakeServer {
    pub(crate) fn packetize_frame(
        &mut self,
        kind: FrameKind,
        keyframe: bool,
        payload: &[u8],
        fec_percent: u8,
    ) -> Vec<Vec<u8>> {
        packetized_frame(self.packetizer(), kind, keyframe, payload, fec_percent)
    }
}
