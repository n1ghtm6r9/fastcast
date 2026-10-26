use crate::link::test::{FakeServer, ReceivedPacket, TEST_WAIT};
use std::net::SocketAddr;
use wire::Hello;

impl FakeServer {
    pub(crate) fn expect_hello(&self) -> (Hello, SocketAddr) {
        loop {
            match self.next_packet(TEST_WAIT) {
                Some((ReceivedPacket::Hello { session: 0, hello }, from)) => return (hello, from),
                Some(_) => continue,
                None => panic!("no Hello with session 0 reached the fake server"),
            }
        }
    }
}
