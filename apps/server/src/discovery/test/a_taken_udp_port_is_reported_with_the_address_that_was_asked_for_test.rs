use crate::discovery::{DiscoveryError, bind_discovery_socket};
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::test]
async fn a_taken_udp_port_is_reported_with_the_address_that_was_asked_for() {
    let first = bind_discovery_socket(SocketAddr::from((Ipv4Addr::LOCALHOST, 0)))
        .await
        .expect("a free port exists");
    let address = first.local_addr().expect("a bound socket has an address");

    let failure = bind_discovery_socket(address)
        .await
        .expect_err("the port is taken");

    assert!(matches!(
        &failure,
        DiscoveryError::Bind { address: asked_for, .. } if *asked_for == address
    ));
    assert!(failure.to_string().contains(&format!("udp {address}")));
}
