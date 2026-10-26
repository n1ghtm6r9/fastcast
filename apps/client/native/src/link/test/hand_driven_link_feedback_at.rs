use crate::link::send_feedback;
use crate::link::test::HandDrivenLink;
use std::time::Duration;

impl HandDrivenLink {
    pub(crate) fn feedback_at(&mut self, at: Duration) {
        send_feedback(
            &self.socket,
            &self.target,
            &self.shared,
            &mut self.state,
            self.start + at,
        );
    }
}
