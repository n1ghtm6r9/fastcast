use super::default_settings;
use crate::capture::open_screen;
use crate::hub::ViewerRequest;
use media::VideoCodec;

#[tokio::test]
async fn a_request_without_codecs_is_served_avc() {
    let screen = open_screen(&default_settings(), &ViewerRequest::default()).await;

    assert_eq!(screen.target.codecs, vec![VideoCodec::Avc]);
}
