use crate::link::test::FakeServer;
use wire::Feedback;

impl FakeServer {
    pub(crate) fn expect_feedback(&self) -> Feedback {
        self.expect_feedback_where(|_| true)
    }
}
