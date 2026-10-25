mod packet;
#[cfg(test)]
mod test_support;

pub use packet::{
    Codec, CodecSupport, DATAGRAM_LIMIT, Datagram, FEEDBACK_SIZE, Feedback, FrameKind, HEADER_SIZE,
    Hello, MAGIC, PROTOCOL_VERSION, Packet, SHARD_HEADER_SIZE, SHARD_SIZE, Shard, ShardHeader,
    Welcome, decode_datagram, encode_datagram,
};
