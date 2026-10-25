use super::raw_datagram;
use crate::packet::decode_datagram;

#[test]
fn a_hello_announcing_more_codecs_than_it_carries_is_ignored() {
    let payload = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 8];

    assert_eq!(decode_datagram(&raw_datagram(1, &payload)), None);
}
