use super::an_hevc_key_frame;
use crate::capture::publish_encoded_frame;
use crate::hub::Hub;
use media::VideoCodec;

#[test]
fn an_encoded_key_frame_describes_the_stream_before_it_is_published() {
    let hub = Hub::new();
    let mut subscription = hub.join();

    publish_encoded_frame(&hub, VideoCodec::Hevc, 120, true, &an_hevc_key_frame(), 7)
        .expect("the key frame describes the stream");

    let frame = subscription
        .frames
        .try_recv()
        .expect("the key frame is published");

    assert!(frame.key);
    assert_eq!(frame.config.codec, "hvc1.1.6.L153.90");
    assert_eq!(
        (frame.config.width, frame.config.height, frame.config.fps),
        (3840, 2160, 120)
    );
    assert_eq!(frame.wire[1..5], 7u32.to_le_bytes());
    assert_eq!(frame.wire[5..], [0, 0, 0, 4, 0x26, 0x01, 0xAF, 0x08]);
}
