use super::default_settings;
use crate::capture::{CaptureBackend, NativeBackend};
use crate::hub::ViewerRequest;
use crate::settings::{CodecChoice, Settings};
use media::VideoCodec;

#[tokio::test]
async fn the_native_backend_encodes_the_chosen_codec_when_the_tv_decodes_it() {
    let settings = Settings {
        video_codec: CodecChoice::Avc,
        ..default_settings()
    };
    let request = ViewerRequest {
        codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
        ..ViewerRequest::default()
    };

    let pipeline = NativeBackend::new(settings)
        .prepare(&request)
        .await
        .expect("the native backend prepares without any work");

    assert_eq!(pipeline.codec, VideoCodec::Avc);
}
