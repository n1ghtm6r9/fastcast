use wire::{Datagrams, FrameKind, OutgoingFrame, Packetizer};

pub(crate) fn packetized_frame(
    packetizer: &mut Packetizer,
    kind: FrameKind,
    keyframe: bool,
    payload: &[u8],
    fec_percent: u8,
) -> Vec<Vec<u8>> {
    let mut datagrams = Datagrams::new();

    packetizer.frame(
        &OutgoingFrame {
            kind,
            keyframe,
            payload,
        },
        fec_percent,
        &mut datagrams,
    );

    datagrams.iter().map(<[u8]>::to_vec).collect()
}
