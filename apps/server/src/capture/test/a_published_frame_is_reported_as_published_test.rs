use super::an_hevc_key_frame;
use crate::capture::publish_encoded_frame;
use crate::hub::Hub;
use media::VideoCodec;

#[test]
fn a_published_frame_is_reported_as_published() {
    let hub = Hub::new();

    let published =
        publish_encoded_frame(&hub, VideoCodec::Hevc, 60, true, &an_hevc_key_frame(), 0)
            .expect("the key frame describes the stream");

    assert!(published);
}
