use crate::packet::{ByteReader, Welcome};

pub fn decode_welcome(reader: &mut ByteReader<'_>) -> Option<Welcome> {
    let nonce = reader.read_u32()?;
    let shard_size = reader.read_u16()?;

    Some(Welcome { nonce, shard_size })
}
