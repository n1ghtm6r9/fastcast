use super::raw_datagram;
use crate::packet::decode_datagram;

#[test]
fn an_unknown_kind_is_ignored() {
    assert_eq!(decode_datagram(&raw_datagram(7, &[])), None);
}
