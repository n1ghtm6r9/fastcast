use super::default_settings;
use crate::capture::{CaptureBackend, NativeBackend};
use crate::hub::ViewerRequest;
use media::VideoCodec;

#[tokio::test]
async fn the_native_backend_takes_av1_only_when_it_is_chosen() {
    let request = ViewerRequest {
        codecs: vec![VideoCodec::Av1, VideoCodec::Hevc, VideoCodec::Avc],
        ..ViewerRequest::default()
    };

    let pipeline = NativeBackend::new(default_settings())
        .prepare(&request)
        .await
        .expect("the native backend prepares without any work");

    assert_eq!(pipeline.codec, VideoCodec::Hevc);
}
