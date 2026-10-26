use crate::link::test::{FakeServer, HandDrivenLink, test_target};
use crate::link::{LinkShared, LinkState};
use std::net::UdpSocket;
use std::time::Instant;

impl HandDrivenLink {
    pub(crate) fn new() -> HandDrivenLink {
        let server = FakeServer::new();
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let target = test_target(server.address());
        let start = Instant::now();

        HandDrivenLink {
            server,
            socket,
            target,
            shared: LinkShared::default(),
            state: LinkState::new(start),
            start,
        }
    }
}
