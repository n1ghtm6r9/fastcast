use std::net::SocketAddr;
use wire::Hello;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkTarget {
    pub server: SocketAddr,
    pub hello: Hello,
}
