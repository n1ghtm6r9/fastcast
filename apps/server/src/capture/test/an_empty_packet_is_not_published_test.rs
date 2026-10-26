use super::an_hevc_key_frame;
use crate::capture::publish_encoded_frame;
use crate::hub::Hub;
use media::VideoCodec;
use tokio::sync::broadcast::error::TryRecvError;

#[test]
fn an_empty_packet_is_not_published() {
    let hub = Hub::new();
    let mut subscription = hub.join();

    publish_encoded_frame(&hub, VideoCodec::Hevc, 60, true, &an_hevc_key_frame(), 0)
        .expect("the key frame describes the stream");
    subscription
        .frames
        .try_recv()
        .expect("the key frame is published");

    publish_encoded_frame(&hub, VideoCodec::Hevc, 60, false, &[], 16)
        .expect("an empty packet is not an error");

    assert!(matches!(
        subscription.frames.try_recv(),
        Err(TryRecvError::Empty)
    ));
}
