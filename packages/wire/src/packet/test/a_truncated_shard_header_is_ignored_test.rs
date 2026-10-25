use super::raw_datagram;
use crate::packet::decode_datagram;

#[test]
fn a_truncated_shard_header_is_ignored() {
    assert_eq!(decode_datagram(&raw_datagram(3, &[0; 17])), None);
}
