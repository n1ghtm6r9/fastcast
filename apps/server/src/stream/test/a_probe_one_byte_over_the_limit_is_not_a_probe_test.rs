use super::probe_of_length;
use crate::stream::is_discovery_probe;

#[test]
fn a_probe_one_byte_over_the_limit_is_not_a_probe() {
    let probe = probe_of_length(1025);

    assert_eq!(probe.len(), 1025);
    assert!(!is_discovery_probe(&probe));
}
