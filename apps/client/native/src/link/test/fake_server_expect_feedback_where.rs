use crate::link::test::{FakeServer, ReceivedPacket, TEST_SESSION, TEST_WAIT};
use std::time::Instant;
use wire::Feedback;

impl FakeServer {
    pub(crate) fn expect_feedback_where(&self, wanted: impl Fn(&Feedback) -> bool) -> Feedback {
        let deadline = Instant::now() + TEST_WAIT;

        while let Some(remaining) = deadline.checked_duration_since(Instant::now()) {
            if let Some((
                ReceivedPacket::Feedback {
                    session: TEST_SESSION,
                    feedback,
                },
                _,
            )) = self.next_packet(remaining)
                && wanted(&feedback)
            {
                return feedback;
            }
        }

        panic!("no Feedback of the test session met the condition");
    }
}
