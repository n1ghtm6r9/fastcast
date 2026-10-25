use crate::discovery::DISCOVERY_PORT;

#[test]
fn the_discovery_port_is_3278_because_the_tv_probes_only_that_port() {
    assert_eq!(DISCOVERY_PORT, 3278);
}
