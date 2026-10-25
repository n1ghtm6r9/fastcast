use crate::packet::{MAGIC, PROTOCOL_VERSION};

pub fn raw_datagram(kind: u8, payload: &[u8]) -> Vec<u8> {
    let mut bytes = vec![MAGIC, PROTOCOL_VERSION, kind, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    bytes.extend_from_slice(payload);

    bytes
}
