use super::default_settings;
use crate::capture::{CaptureBackend, NativeBackend};
use crate::hub::ViewerRequest;
use crate::settings::{CodecChoice, Settings};
use media::VideoCodec;

#[tokio::test]
async fn the_native_backend_encodes_av1_when_it_is_chosen_and_the_tv_decodes_it() {
    let settings = Settings {
        video_codec: CodecChoice::Av1,
        ..default_settings()
    };
    let request = ViewerRequest {
        codecs: vec![VideoCodec::Hevc, VideoCodec::Av1, VideoCodec::Avc],
        ..ViewerRequest::default()
    };

    let pipeline = NativeBackend::new(settings)
        .prepare(&request)
        .await
        .expect("the native backend prepares without any work");

    assert_eq!(pipeline.codec, VideoCodec::Av1);
}
