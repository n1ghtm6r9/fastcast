use super::default_settings;
use crate::capture::{CaptureBackend, NativeBackend};
use crate::hub::ViewerRequest;
use media::VideoCodec;

#[tokio::test]
async fn the_native_backend_uses_avc_when_the_tv_lists_no_hevc() {
    let request = ViewerRequest {
        codecs: Vec::new(),
        ..ViewerRequest::default()
    };

    let pipeline = NativeBackend::new(default_settings())
        .prepare(&request)
        .await
        .expect("the native backend prepares without any work");

    assert_eq!(pipeline.codec, VideoCodec::Avc);
}
