use crate::flv::{
    COMPOSITION_TIME_LENGTH, ENHANCED_CODED_FRAMES_PACKET, ENHANCED_CODED_FRAMES_X_PACKET,
    ENHANCED_FRAME_TYPE_MASK, ENHANCED_PACKET_TYPE_MASK, ENHANCED_SEQUENCE_START_PACKET,
    ENHANCED_VIDEO_TAG_HEADER_LENGTH, FRAME_TYPE_SHIFT, FlvEvent, KEY_FRAME_TYPE,
    video_codec_of_fourcc,
};
use bytes::Bytes;

pub fn parse_enhanced_video_tag(timestamp_ms: u32, data: Bytes) -> Option<FlvEvent> {
    if data.len() < ENHANCED_VIDEO_TAG_HEADER_LENGTH {
        return None;
    }

    let frame_type = (data[0] & ENHANCED_FRAME_TYPE_MASK) >> FRAME_TYPE_SHIFT;
    let packet_type = data[0] & ENHANCED_PACKET_TYPE_MASK;
    let codec = video_codec_of_fourcc([data[1], data[2], data[3], data[4]])?;
    let payload = data.slice(ENHANCED_VIDEO_TAG_HEADER_LENGTH..);
    let key = frame_type == KEY_FRAME_TYPE;

    match packet_type {
        ENHANCED_SEQUENCE_START_PACKET => Some(FlvEvent::SequenceHeader {
            codec,
            record: payload,
        }),
        ENHANCED_CODED_FRAMES_X_PACKET => Some(FlvEvent::Frame {
            key,
            timestamp_ms,
            data: payload,
        }),
        ENHANCED_CODED_FRAMES_PACKET if payload.len() >= COMPOSITION_TIME_LENGTH => {
            Some(FlvEvent::Frame {
                key,
                timestamp_ms,
                data: payload.slice(COMPOSITION_TIME_LENGTH..),
            })
        }
        _ => None,
    }
}
