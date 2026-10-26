use crate::stream::{StreamError, bind_stream_socket};
use std::net::{Ipv4Addr, SocketAddr};

#[test]
fn a_taken_udp_port_is_reported_with_the_address_that_was_asked_for() {
    let first =
        bind_stream_socket(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))).expect("a free port exists");
    let address = first.local_addr().expect("a bound socket has an address");

    let failure = bind_stream_socket(address).expect_err("the port is taken");

    assert!(matches!(
        &failure,
        StreamError::Bind { address: asked_for, .. } if *asked_for == address
    ));
    assert!(failure.to_string().contains(&format!("udp {address}")));
}
