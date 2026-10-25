use crate::packet::FrameKind;
use crate::packetizer::{Datagrams, OutgoingFrame, Packetizer};

pub fn datagrams_of(
    packetizer: &mut Packetizer,
    kind: FrameKind,
    keyframe: bool,
    payload: &[u8],
    fec_percent: u8,
) -> Vec<Vec<u8>> {
    let mut out = Datagrams::new();

    packetizer.frame(
        &OutgoingFrame {
            kind,
            keyframe,
            payload,
        },
        fec_percent,
        &mut out,
    );

    out.iter().map(<[u8]>::to_vec).collect()
}
