use super::an_avc_key_frame;
use crate::capture::publish_encoded_frame;
use crate::hub::Hub;
use media::VideoCodec;

#[test]
fn an_avc_key_frame_describes_the_stream_before_it_is_published() {
    let hub = Hub::new();
    let mut subscription = hub.join();

    publish_encoded_frame(
        &hub,
        VideoCodec::Avc,
        60,
        true,
        &an_avc_key_frame(),
        7,
        None,
    )
    .expect("the key frame describes the stream");

    let frame = subscription
        .frames
        .try_recv()
        .expect("the key frame is published");

    assert!(frame.key);
    assert_eq!(frame.config.codec, "avc1.640033");
    assert_eq!(
        (frame.config.width, frame.config.height, frame.config.fps),
        (2560, 1440, 60)
    );
    assert_eq!(frame.wire[1..5], 7u32.to_le_bytes());
    assert_eq!(frame.wire[5..], [0, 0, 0, 4, 0x65, 0x88, 0x84, 0x21]);
}
