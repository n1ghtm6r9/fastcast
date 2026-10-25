mod fec;
mod gf;
mod layout;
mod packet;
mod packetizer;
mod receiver;
#[cfg(test)]
mod test_support;

pub use fec::{MAX_DATA_SHARDS, MAX_PARITY_SHARDS};
pub use layout::{FrameLayout, MAX_FRAME_SIZE, parity_shards_for};
pub use packet::{
    Codec, CodecSupport, DATAGRAM_LIMIT, Datagram, FEEDBACK_SIZE, Feedback, FrameKind, HEADER_SIZE,
    Hello, MAGIC, PROTOCOL_VERSION, Packet, SHARD_HEADER_SIZE, SHARD_SIZE, Shard, ShardHeader,
    Welcome, decode_datagram, encode_datagram,
};
pub use packetizer::{Datagrams, OutgoingFrame, Packetizer};
pub use receiver::{
    FRAME_TIMEOUT, FrameReceiver, MAX_PENDING_FRAMES, REORDER_WINDOW, ReceivedFrame,
};
