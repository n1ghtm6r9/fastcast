use crate::stream::StreamEndpoint;
use std::net::SocketAddr;

impl StreamEndpoint {
    pub fn local_addr(&self) -> SocketAddr {
        self.local
    }
}
