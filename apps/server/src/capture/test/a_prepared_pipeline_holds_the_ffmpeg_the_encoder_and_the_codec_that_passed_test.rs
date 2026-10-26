use super::{FakeFfmpeg, ffmpeg_backend_for};
use crate::capture::{CaptureBackend, Pipeline, VideoEncoder};
use crate::hub::ViewerRequest;
use media::VideoCodec;

#[tokio::test]
async fn a_prepared_pipeline_holds_the_ffmpeg_the_encoder_and_the_codec_that_passed() {
    let ffmpeg = FakeFfmpeg::exiting_at_once();
    let request = ViewerRequest {
        codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
        ..ViewerRequest::default()
    };

    let pipeline = ffmpeg_backend_for(&ffmpeg)
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
