use super::annex_b_of;
use crate::capture::publish_encoded_frame;
use crate::hub::Hub;
use media::VideoCodec;
use tokio::sync::broadcast::error::TryRecvError;

#[test]
fn a_delta_frame_before_any_key_frame_is_not_published() {
    let hub = Hub::new();
    let mut subscription = hub.join();

    publish_encoded_frame(
        &hub,
        VideoCodec::Hevc,
        60,
        false,
        &annex_b_of(&[&[0x02, 0x01, 0xD0]]),
        0,
        None,
    )
    .expect("a delta frame needs no description");

    assert!(matches!(
        subscription.frames.try_recv(),
        Err(TryRecvError::Empty)
    ));
}
