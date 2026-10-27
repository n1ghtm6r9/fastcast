use crate::link::test::{FakeServer, ReceivedPacket, TEST_SESSION, TEST_WAIT};

impl FakeServer {
    pub(crate) fn expect_notice(&self) -> String {
        loop {
            match self.next_packet(TEST_WAIT) {
                Some((
                    ReceivedPacket::Notice {
                        session: TEST_SESSION,
                        text,
                    },
                    _,
                )) => return text,
                Some(_) => continue,
                None => panic!("no Notice of the test session reached the fake server"),
            }
        }
    }
}
