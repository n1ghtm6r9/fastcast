use super::{hub_with_x264_config, next_frame, next_welcome, publish_gop, start_session};
use crate::hub::ViewerRequest;
use crate::stream::MAX_REPLAY_FRAMES;
use wire::{FrameKind, FrameReceiver, ReceivedFrame};

#[test]
fn a_session_joining_a_gop_of_exactly_the_replay_limit_replays_it() {
    let hub = hub_with_x264_config();

    publish_gop(&hub, MAX_REPLAY_FRAMES - 1);

    let session = start_session(&hub, ViewerRequest::default());

    next_welcome(&session.peer);

    let mut receiver = FrameReceiver::new();
    let frames: Vec<ReceivedFrame> = (0..=MAX_REPLAY_FRAMES)
        .map(|_| next_frame(&session.peer, &mut receiver))
        .collect();

    assert_eq!(frames[0].kind, FrameKind::Config);
    assert!(frames[1].keyframe);
    assert!(frames.iter().skip(2).all(|frame| !frame.keyframe));
    assert!(
        frames
            .iter()
            .enumerate()
            .all(|(index, frame)| frame.number as usize == index)
    );

    session.stop_and_join();

    assert!(!hub.take_key_frame_request());
}
