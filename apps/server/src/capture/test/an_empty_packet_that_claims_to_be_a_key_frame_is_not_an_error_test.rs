use crate::capture::publish_encoded_frame;
use crate::hub::Hub;
use media::VideoCodec;

#[test]
fn an_empty_packet_that_claims_to_be_a_key_frame_is_not_an_error() {
    let hub = Hub::new();

    let result = publish_encoded_frame(&hub, VideoCodec::Hevc, 60, true, &[], 0, None);

    assert!(result.is_ok());
}
