use super::default_settings;
use crate::capture::FfmpegBackend;
use media::VideoCodec;

#[test]
fn ffmpeg_has_encoders_for_avc_and_hevc_only() {
    let backend = FfmpegBackend::new(default_settings());

    assert!(backend.picker_for(VideoCodec::Avc).is_some());
    assert!(backend.picker_for(VideoCodec::Hevc).is_some());
    assert!(backend.picker_for(VideoCodec::Av1).is_none());
}
