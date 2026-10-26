use crate::link::unspecified_address_for;
use std::net::SocketAddr;

#[test]
fn the_local_address_has_the_family_of_the_server() {
    let v4: SocketAddr = "192.168.0.220:3278".parse().unwrap();
    let v6: SocketAddr = "[fe80::1]:3278".parse().unwrap();

    assert_eq!(
        unspecified_address_for(&v4),
        "0.0.0.0:0".parse::<SocketAddr>().unwrap()
    );
    assert_eq!(
        unspecified_address_for(&v6),
        "[::]:0".parse::<SocketAddr>().unwrap()
    );
}
