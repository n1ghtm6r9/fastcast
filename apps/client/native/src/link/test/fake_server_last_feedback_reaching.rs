use crate::link::test::{FakeServer, ReceivedPacket, TEST_SESSION, TEST_WAIT};
use std::time::Instant;
use wire::Feedback;

impl FakeServer {
    pub(crate) fn last_feedback_reaching(&self, sequence: u32) -> Feedback {
        let deadline = Instant::now() + TEST_WAIT;
        let mut last = None;

        while let Some(remaining) = deadline.checked_duration_since(Instant::now()) {
            if let Some((
                ReceivedPacket::Feedback {
                    session: TEST_SESSION,
                    feedback,
                },
                _,
            )) = self.next_packet(remaining)
            {
                let reached = feedback.highest_sequence >= sequence;

                last = Some(feedback);

                if reached {
                    break;
                }
            }
        }

        last.expect("no Feedback of the test session reached the fake server")
    }
}
