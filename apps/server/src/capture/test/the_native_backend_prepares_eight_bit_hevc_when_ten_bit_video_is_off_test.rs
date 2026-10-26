use super::default_settings;
use crate::capture::{CaptureBackend, NativeBackend, NativePipeline};
use crate::hub::{EIGHT_BIT_DEPTH, ViewerRequest};
use crate::settings::Settings;
use media::VideoCodec;

#[tokio::test]
async fn the_native_backend_prepares_eight_bit_hevc_when_ten_bit_video_is_off() {
    let settings = Settings {
        ten_bit_video: false,
        ..default_settings()
    };
    let request = ViewerRequest {
        codecs: vec![VideoCodec::Hevc],
        ten_bit_codecs: vec![VideoCodec::Hevc],
        ..ViewerRequest::default()
    };

    let pipeline = NativeBackend::new(settings)
        .prepare(&request)
        .await
        .expect("the native backend prepares without any work");

    assert_eq!(
        pipeline,
        NativePipeline {
            codec: VideoCodec::Hevc,
            bit_depth: EIGHT_BIT_DEPTH,
        }
    );
}
