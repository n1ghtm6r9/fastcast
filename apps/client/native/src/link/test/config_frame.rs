use wire::{FrameKind, ReceivedFrame};

pub(crate) fn config_frame(payload: &[u8]) -> ReceivedFrame {
    ReceivedFrame {
        number: 0,
        kind: FrameKind::Config,
        keyframe: false,
        payload: payload.to_vec(),
    }
}
