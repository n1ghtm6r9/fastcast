use crate::packet::decode_datagram;

pub fn sequence_of(bytes: &[u8]) -> u32 {
    decode_datagram(bytes)
        .expect("a decodable datagram")
        .sequence
}
