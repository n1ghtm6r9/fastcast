use crate::link::test::FakeServer;
use crate::link::{LinkShared, LinkState, LinkTarget};
use std::net::UdpSocket;
use std::time::Instant;

pub(crate) struct HandDrivenLink {
    pub(crate) server: FakeServer,
    pub(crate) socket: UdpSocket,
    pub(crate) target: LinkTarget,
    pub(crate) shared: LinkShared,
    pub(crate) state: LinkState,
    pub(crate) start: Instant,
}
