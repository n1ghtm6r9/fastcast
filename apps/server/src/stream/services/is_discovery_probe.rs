use crate::stream::{MAX_PROBE_BYTES, PROBE_KIND};
use serde_json::Value;

pub fn is_discovery_probe(datagram: &[u8]) -> bool {
    datagram.len() <= MAX_PROBE_BYTES
        && serde_json::from_slice::<Value>(datagram).is_ok_and(|probe| probe["type"] == PROBE_KIND)
}
