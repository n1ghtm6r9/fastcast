use crate::support::grow_receive_buffer;
use std::net::{Ipv4Addr, UdpSocket};

pub fn bind_peer() -> UdpSocket {
    let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("a free port exists");

    grow_receive_buffer(&socket);

    socket
}
