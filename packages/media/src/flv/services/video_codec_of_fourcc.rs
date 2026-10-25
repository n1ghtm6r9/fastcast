use crate::flv::{AVC_FOURCC, HEVC_FOURCC, VideoCodec};

pub fn video_codec_of_fourcc(fourcc: [u8; 4]) -> Option<VideoCodec> {
    match fourcc {
        AVC_FOURCC => Some(VideoCodec::Avc),
        HEVC_FOURCC => Some(VideoCodec::Hevc),
        _ => None,
    }
}
