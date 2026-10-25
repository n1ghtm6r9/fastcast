use crate::flv::VideoCodec;
use bytes::Bytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlvEvent {
    SequenceHeader {
        codec: VideoCodec,
        record: Bytes,
    },
    Frame {
        key: bool,
        timestamp_ms: u32,
        data: Bytes,
    },
}
