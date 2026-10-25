use crate::web::host_with_port;

#[test]
fn a_name_or_ipv4_host_is_joined_to_the_port_as_it_is() {
    assert_eq!(host_with_port("0.0.0.0", 3278), "0.0.0.0:3278");
    assert_eq!(host_with_port("localhost", 80), "localhost:80");
}
