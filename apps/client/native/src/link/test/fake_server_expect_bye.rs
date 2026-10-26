use crate::link::test::{FakeServer, ReceivedPacket, TEST_WAIT};

impl FakeServer {
    pub(crate) fn expect_bye(&self) -> u32 {
        loop {
            match self.next_packet(TEST_WAIT) {
                Some((ReceivedPacket::Bye { session }, _)) => return session,
                Some(_) => continue,
                None => panic!("no Bye reached the fake server"),
            }
        }
    }
}
