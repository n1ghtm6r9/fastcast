use std::net::{Ipv4Addr, UdpSocket};

pub fn bind_peer() -> UdpSocket {
    UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("a free port exists")
}
