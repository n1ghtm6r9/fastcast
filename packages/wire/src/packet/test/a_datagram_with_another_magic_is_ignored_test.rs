use super::raw_datagram;
use crate::packet::decode_datagram;

#[test]
fn a_datagram_with_another_magic_is_ignored() {
    let mut bytes = raw_datagram(5, &[]);
    bytes[0] = 0xFB;

    assert_eq!(decode_datagram(&bytes), None);
}
