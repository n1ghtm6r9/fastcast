use super::raw_datagram;
use crate::packet::decode_datagram;

#[test]
fn a_datagram_shorter_than_the_header_is_ignored() {
    let bytes = raw_datagram(5, &[]);

    assert_eq!(decode_datagram(&bytes[..11]), None);
}
