use super::{FakeFfmpeg, default_settings};
use crate::capture::{CaptureBackend, FfmpegBackend};
use crate::hub::ViewerRequest;
use crate::settings::Settings;
use media::VideoCodec;

#[tokio::test]
async fn a_request_without_codecs_is_prepared_for_avc() {
    let ffmpeg = FakeFfmpeg::exiting_at_once();
    let settings = Settings {
        ffmpeg_path: ffmpeg.path(),
        ..default_settings()
    };

    let pipeline = FfmpegBackend::new(settings)
        .prepare(&ViewerRequest::default())
        .await
        .expect("the fake ffmpeg passes its probe");

    assert_eq!(pipeline.codec, VideoCodec::Avc);
}
