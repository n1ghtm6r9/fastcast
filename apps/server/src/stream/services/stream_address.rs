use crate::stream::LISTEN_IP;
use std::net::SocketAddr;

pub fn stream_address(port: u16) -> SocketAddr {
    SocketAddr::from((LISTEN_IP, port))
}
