use crate::link::test::{FakeServer, ReceivedPacket, TEST_SESSION, TEST_WAIT};

impl FakeServer {
    pub(crate) fn expect_stats(&self) -> String {
        loop {
            match self.next_packet(TEST_WAIT) {
                Some((
                    ReceivedPacket::Stats {
                        session: TEST_SESSION,
                        text,
                    },
                    _,
                )) => return text,
                Some(_) => continue,
                None => panic!("no Stats of the test session reached the fake server"),
            }
        }
    }
}
