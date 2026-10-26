use crate::capture::publish_encoded_frame;
use crate::hub::Hub;
use media::VideoCodec;

#[test]
fn an_empty_packet_is_reported_as_not_published() {
    let hub = Hub::new();

    let published = publish_encoded_frame(&hub, VideoCodec::Hevc, 60, false, &[], 0)
        .expect("an empty packet is not an error");

    assert!(!published);
}
