use crate::support::{TestViewer, a_bare_hello};
use std::net::SocketAddr;

pub fn connect_viewers(server: SocketAddr, count: usize) -> Vec<TestViewer> {
    (0..count)
        .map(|_| TestViewer::connect(server, a_bare_hello()))
        .collect()
}
