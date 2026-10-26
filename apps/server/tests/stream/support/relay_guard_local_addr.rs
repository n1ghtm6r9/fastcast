use crate::support::RelayGuard;
use std::net::SocketAddr;

impl RelayGuard {
    pub fn local_addr(&self) -> SocketAddr {
        self.local
    }
}
