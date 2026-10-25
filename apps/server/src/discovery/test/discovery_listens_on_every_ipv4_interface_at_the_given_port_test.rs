use crate::discovery::discovery_address;

#[test]
fn discovery_listens_on_every_ipv4_interface_at_the_given_port() {
    assert_eq!(discovery_address(3278).to_string(), "0.0.0.0:3278");
    assert_eq!(discovery_address(4000).to_string(), "0.0.0.0:4000");
}
