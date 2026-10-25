use crate::discovery::LISTEN_IP;
use std::net::SocketAddr;

pub fn discovery_address(port: u16) -> SocketAddr {
    SocketAddr::from((LISTEN_IP, port))
}
