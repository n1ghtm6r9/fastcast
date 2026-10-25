use crate::flv::{ENHANCED_HEADER_FLAG, FlvEvent, parse_avc_video_tag, parse_enhanced_video_tag};
use bytes::Bytes;

pub fn parse_video_tag(timestamp_ms: u32, data: Bytes) -> Option<FlvEvent> {
    match data.first() {
        Some(first) if first & ENHANCED_HEADER_FLAG != 0 => {
            parse_enhanced_video_tag(timestamp_ms, data)
        }
        _ => parse_avc_video_tag(timestamp_ms, data),
    }
}
