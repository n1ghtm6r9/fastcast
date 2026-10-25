use super::{FakeFfmpeg, default_settings};
use crate::capture::{CaptureBackend, FfmpegBackend, Pipeline};
use crate::hub::ViewerRequest;
use crate::settings::{Settings, VideoEncoder};
use media::VideoCodec;

#[tokio::test]
async fn a_prepared_pipeline_holds_the_ffmpeg_the_encoder_and_the_codec_that_passed() {
    let ffmpeg = FakeFfmpeg::exiting_at_once();
    let settings = Settings {
        ffmpeg_path: ffmpeg.path(),
        ..default_settings()
    };
    let request = ViewerRequest {
        codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
        ..ViewerRequest::default()
    };

    let pipeline = FfmpegBackend::new(settings)
        .prepare(&request)
        .await
        .expect("the fake ffmpeg passes its probe");

    assert_eq!(
        pipeline,
        Pipeline {
            ffmpeg_path: ffmpeg.path(),
            encoder: VideoEncoder::Nvenc,
            codec: VideoCodec::Hevc,
        }
    );
}
