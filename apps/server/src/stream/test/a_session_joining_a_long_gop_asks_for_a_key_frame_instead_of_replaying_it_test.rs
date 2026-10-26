use super::{
    TEST_SESSION, hub_with_x264_config, next_bye, next_frame, next_welcome, publish_gop,
    start_session,
};
use crate::hub::ViewerRequest;
use wire::{FrameKind, FrameReceiver};

#[test]
fn a_session_joining_a_long_gop_asks_for_a_key_frame_instead_of_replaying_it() {
    let hub = hub_with_x264_config();

    publish_gop(&hub, 30);

    let session = start_session(&hub, ViewerRequest::default());

    next_welcome(&session.peer);

    let mut receiver = FrameReceiver::new();
    let config = next_frame(&session.peer, &mut receiver);

    assert_eq!(config.kind, FrameKind::Config);

    let peer = session.stop_and_join();

    assert_eq!(next_bye(&peer), TEST_SESSION);
    assert!(hub.take_key_frame_request());
}
