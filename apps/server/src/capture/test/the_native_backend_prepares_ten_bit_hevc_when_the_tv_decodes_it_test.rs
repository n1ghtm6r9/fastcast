use super::default_settings;
use crate::capture::{CaptureBackend, NativeBackend, NativePipeline};
use crate::hub::{TEN_BIT_DEPTH, ViewerRequest};
use media::VideoCodec;

#[tokio::test]
async fn the_native_backend_prepares_ten_bit_hevc_when_the_tv_decodes_it() {
    let request = ViewerRequest {
        codecs: vec![VideoCodec::Hevc],
        ten_bit_codecs: vec![VideoCodec::Hevc],
        ..ViewerRequest::default()
    };

    let pipeline = NativeBackend::new(default_settings())
        .prepare(&request)
        .await
        .expect("the native backend prepares without any work");

    assert_eq!(
        pipeline,
        NativePipeline {
            codec: VideoCodec::Hevc,
            bit_depth: TEN_BIT_DEPTH,
        }
    );
}
