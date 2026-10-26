use crate::link::test::FakeServer;
use std::net::SocketAddr;
use wire::FrameKind;

impl FakeServer {
    pub(crate) fn send_frame(
        &mut self,
        peer: SocketAddr,
        kind: FrameKind,
        keyframe: bool,
        payload: &[u8],
        fec_percent: u8,
    ) -> Vec<Vec<u8>> {
        let datagrams = self.packetize_frame(kind, keyframe, payload, fec_percent);

        self.send_datagrams(peer, &datagrams);

        datagrams
    }
}
