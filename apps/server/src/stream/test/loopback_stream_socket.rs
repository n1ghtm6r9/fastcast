use crate::stream::bind_stream_socket;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};

pub fn loopback_stream_socket() -> UdpSocket {
    bind_stream_socket(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))).expect("a free port exists")
}
