use crate::stream::bind_stream_socket;
use std::net::{Ipv4Addr, SocketAddr};

#[test]
fn binding_port_zero_picks_a_free_udp_port() {
    let socket =
        bind_stream_socket(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))).expect("a free port exists");
    let address = socket.local_addr().expect("a bound socket has an address");

    assert_ne!(address.port(), 0);
    assert!(address.ip().is_loopback());
}
