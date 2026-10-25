use super::raw_datagram;
use crate::packet::decode_datagram;

#[test]
fn an_unknown_frame_kind_is_ignored() {
    let mut payload = [0; 18];
    payload[4] = 2;

    assert_eq!(decode_datagram(&raw_datagram(3, &payload)), None);
}
