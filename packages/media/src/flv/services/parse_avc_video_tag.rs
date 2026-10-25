use crate::flv::{
    AVC_CODEC_ID, AVC_NALU_PACKET, AVC_SEQUENCE_HEADER_PACKET, AVC_VIDEO_TAG_HEADER_LENGTH,
    FlvEvent, KEY_FRAME_TYPE,
};
use bytes::Bytes;

pub fn parse_avc_video_tag(timestamp_ms: u32, data: Bytes) -> Option<FlvEvent> {
    if data.len() < AVC_VIDEO_TAG_HEADER_LENGTH {
        return None;
    }

    let frame_type = data[0] >> 4;
    let codec_id = data[0] & 0x0F;

    if codec_id != AVC_CODEC_ID {
        return None;
    }

    let payload = data.slice(AVC_VIDEO_TAG_HEADER_LENGTH..);

    match data[1] {
        AVC_SEQUENCE_HEADER_PACKET => Some(FlvEvent::SequenceHeader(payload)),
        AVC_NALU_PACKET => Some(FlvEvent::Frame {
            key: frame_type == KEY_FRAME_TYPE,
            timestamp_ms,
            data: payload,
        }),
        _ => None,
    }
}
