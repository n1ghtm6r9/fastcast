use crate::link::deliver_frames;
use crate::link::test::HandDrivenLink;
use std::time::Duration;

impl HandDrivenLink {
    pub(crate) fn deliver_at(&mut self, at: Duration) {
        deliver_frames(&self.shared, &mut self.state, self.start + at);
    }
}
