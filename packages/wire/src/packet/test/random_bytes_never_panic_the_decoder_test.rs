use crate::{
    packet::{MAGIC, PROTOCOL_VERSION, decode_datagram},
    test_support::{random_bytes, xorshift},
};

#[test]
fn random_bytes_never_panic_the_decoder() {
    let mut state = 0x9E37_79B9_7F4A_7C15;

    for _ in 0..10_000 {
        let length = (xorshift(&mut state) % 1501) as usize;
        let kind = (xorshift(&mut state) % 8) as u8;
        let mut bytes = vec![MAGIC, PROTOCOL_VERSION, kind];
        bytes.extend(random_bytes(&mut state, length));

        decode_datagram(&bytes);
    }
}
