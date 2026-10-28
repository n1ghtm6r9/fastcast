use super::{annex_b_of, hvcc_parameter_sets_of};
use crate::capture::{NativeError, publish_encoded_frame};
use crate::fixtures::nvenc_3840x2160_hvcc;
use crate::hub::Hub;
use media::VideoCodec;
use tokio::sync::broadcast::error::TryRecvError;

#[test]
fn a_key_frame_whose_parameter_sets_cannot_be_described_is_refused() {
    let hub = Hub::new();
    let mut subscription = hub.join();
    let [vps, _, pps] = hvcc_parameter_sets_of(&nvenc_3840x2160_hvcc());

    let result = publish_encoded_frame(
        &hub,
        VideoCodec::Hevc,
        60,
        true,
        &annex_b_of(&[&vps, &[0x42, 0x01, 0x01], &pps, &[0x26, 0x01, 0xAF, 0x08]]),
        0,
        None,
    );

    assert!(matches!(result, Err(NativeError::UndescribableStream)));
    assert!(matches!(
        subscription.frames.try_recv(),
        Err(TryRecvError::Empty)
    ));
}
