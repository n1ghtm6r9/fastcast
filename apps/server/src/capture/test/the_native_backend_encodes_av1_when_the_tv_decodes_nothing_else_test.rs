use super::default_settings;
use crate::capture::{CaptureBackend, NativeBackend};
use crate::hub::ViewerRequest;
use media::VideoCodec;

#[tokio::test]
async fn the_native_backend_encodes_av1_when_the_tv_decodes_nothing_else() {
    let request = ViewerRequest {
        codecs: vec![VideoCodec::Av1],
        ..ViewerRequest::default()
    };

    let pipeline = NativeBackend::new(default_settings())
        .prepare(&request)
        .await
        .expect("the native backend prepares without any work");

    assert_eq!(pipeline.codec, VideoCodec::Av1);
}
