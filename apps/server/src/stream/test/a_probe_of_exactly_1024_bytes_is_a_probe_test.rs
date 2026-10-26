use super::probe_of_length;
use crate::stream::is_discovery_probe;

#[test]
fn a_probe_of_exactly_1024_bytes_is_a_probe() {
    let probe = probe_of_length(1024);

    assert_eq!(probe.len(), 1024);
    assert!(is_discovery_probe(&probe));
}
