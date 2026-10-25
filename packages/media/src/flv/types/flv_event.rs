use bytes::Bytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlvEvent {
    SequenceHeader(Bytes),
    Frame {
        key: bool,
        timestamp_ms: u32,
        data: Bytes,
    },
}
