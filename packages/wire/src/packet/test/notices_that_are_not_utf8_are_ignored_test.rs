use super::raw_datagram;
use crate::packet::decode_datagram;

#[test]
fn notices_that_are_not_utf8_are_ignored() {
    assert_eq!(decode_datagram(&raw_datagram(7, &[0xFF])), None);
}
