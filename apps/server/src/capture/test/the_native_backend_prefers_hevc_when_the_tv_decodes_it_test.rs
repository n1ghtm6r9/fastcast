use super::default_settings;
use crate::capture::{CaptureBackend, NativeBackend};
use crate::hub::ViewerRequest;
use media::VideoCodec;

#[tokio::test]
async fn the_native_backend_prefers_hevc_when_the_tv_decodes_it() {
    let request = ViewerRequest {
        codecs: vec![VideoCodec::Avc, VideoCodec::Hevc],
        ..ViewerRequest::default()
    };

    let pipeline = NativeBackend::new(default_settings())
        .prepare(&request)
        .await
        .expect("the native backend prepares without any work");

    assert_eq!(pipeline.codec, VideoCodec::Hevc);
}
