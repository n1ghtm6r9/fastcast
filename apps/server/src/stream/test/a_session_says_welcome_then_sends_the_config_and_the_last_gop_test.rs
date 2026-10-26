use super::{
    TEST_NONCE, TEST_SESSION, hub_with_x264_config, next_bye, next_frame, next_welcome,
    publish_gop, start_session,
};
use crate::hub::{Subscription, ViewerRequest};
use wire::{FrameKind, FrameReceiver, ReceivedFrame, SHARD_SIZE, Welcome};

#[test]
fn a_session_says_welcome_then_sends_the_config_and_the_last_gop() {
    let hub = hub_with_x264_config();

    publish_gop(&hub, 2);

    let Subscription { config, gop, .. } = hub.join();
    let config = config.expect("the hub has a config");
    let session = start_session(&hub, ViewerRequest::default());

    let (session_id, welcome) = next_welcome(&session.peer);

    assert_eq!(session_id, TEST_SESSION);
    assert_eq!(
        welcome,
        Welcome {
            nonce: TEST_NONCE,
            shard_size: SHARD_SIZE as u16
        }
    );

    let mut receiver = FrameReceiver::new();
    let frames: Vec<ReceivedFrame> = (0..4)
        .map(|_| next_frame(&session.peer, &mut receiver))
        .collect();

    assert_eq!(
        frames,
        vec![
            ReceivedFrame {
                number: 0,
                kind: FrameKind::Config,
                keyframe: false,
                payload: config.json.as_bytes().to_vec(),
            },
            ReceivedFrame {
                number: 1,
                kind: FrameKind::Video,
                keyframe: true,
                payload: gop[0].wire.to_vec(),
            },
            ReceivedFrame {
                number: 2,
                kind: FrameKind::Video,
                keyframe: false,
                payload: gop[1].wire.to_vec(),
            },
            ReceivedFrame {
                number: 3,
                kind: FrameKind::Video,
                keyframe: false,
                payload: gop[2].wire.to_vec(),
            },
        ]
    );

    let peer = session.stop_and_join();

    assert_eq!(next_bye(&peer), TEST_SESSION);
}
