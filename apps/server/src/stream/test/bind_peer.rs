use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;

pub fn bind_peer() -> UdpSocket {
    let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("a free port exists");

    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("a read timeout can be set");

    socket
}
