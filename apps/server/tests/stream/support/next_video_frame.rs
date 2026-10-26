use crate::support::{ARRIVAL_LIMIT, TestViewer, VideoFrame, received_frame_of};
use wire::FrameKind;

pub fn next_video_frame(viewer: &mut TestViewer) -> VideoFrame {
    let frame = viewer
        .next_frame(ARRIVAL_LIMIT)
        .expect("a video frame arrives in time");

    assert_eq!(frame.kind, FrameKind::Video, "expected a video frame");

    received_frame_of(&frame)
}
