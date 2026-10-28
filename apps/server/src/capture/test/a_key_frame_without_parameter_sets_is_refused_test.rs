use super::annex_b_of;
use crate::capture::{NativeError, publish_encoded_frame};
use crate::hub::Hub;
use media::VideoCodec;
use tokio::sync::broadcast::error::TryRecvError;

#[test]
fn a_key_frame_without_parameter_sets_is_refused() {
    let hub = Hub::new();
    let mut subscription = hub.join();

    let result = publish_encoded_frame(
        &hub,
        VideoCodec::Hevc,
        60,
        true,
        &annex_b_of(&[&[0x26, 0x01, 0xAF, 0x08]]),
        0,
        None,
    );

    assert!(matches!(result, Err(NativeError::UndescribableStream)));
    assert!(matches!(
        subscription.frames.try_recv(),
        Err(TryRecvError::Empty)
    ));
}
