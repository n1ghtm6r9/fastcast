use super::{FakeFfmpeg, ffmpeg_backend_for};
use crate::capture::CaptureBackend;
use crate::hub::ViewerRequest;
use media::VideoCodec;

#[tokio::test]
async fn a_request_without_codecs_is_prepared_for_avc() {
    let ffmpeg = FakeFfmpeg::exiting_at_once();
    let pipeline = ffmpeg_backend_for(&ffmpeg)
        .prepare(&ViewerRequest::default())
        .await
        .expect("the fake ffmpeg passes its probe");

    assert_eq!(pipeline.codec, VideoCodec::Avc);
}
