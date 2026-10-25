use crate::packet::Welcome;

pub fn write_welcome(welcome: &Welcome, out: &mut Vec<u8>) {
    out.extend_from_slice(&welcome.nonce.to_le_bytes());
    out.extend_from_slice(&welcome.shard_size.to_le_bytes());
}
