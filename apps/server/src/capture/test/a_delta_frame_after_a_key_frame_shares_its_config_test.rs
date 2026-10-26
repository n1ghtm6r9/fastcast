use super::{an_hevc_key_frame, annex_b_of};
use crate::capture::publish_encoded_frame;
use crate::hub::Hub;
use media::VideoCodec;
use std::sync::Arc;

#[test]
fn a_delta_frame_after_a_key_frame_shares_its_config() {
    let hub = Hub::new();
    let mut subscription = hub.join();

    publish_encoded_frame(&hub, VideoCodec::Hevc, 60, true, &an_hevc_key_frame(), 0)
        .expect("the key frame describes the stream");

    publish_encoded_frame(
        &hub,
        VideoCodec::Hevc,
        60,
        false,
        &annex_b_of(&[&[0x02, 0x01, 0xD0]]),
        16,
    )
    .expect("a delta frame needs no description");

    let key = subscription
        .frames
        .try_recv()
        .expect("the key frame is published");

    let delta = subscription
        .frames
        .try_recv()
        .expect("the delta frame is published");

    assert!(!delta.key);
    assert!(Arc::ptr_eq(&key.config, &delta.config));
    assert_eq!(delta.wire[5..], [0, 0, 0, 3, 0x02, 0x01, 0xD0]);
}
