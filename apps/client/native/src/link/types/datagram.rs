use std::net::SocketAddr;

#[derive(Debug)]
pub struct Datagram {
    pub bytes: Vec<u8>,
    pub from: SocketAddr,
}
