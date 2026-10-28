use crate::link::note_turn;
use crate::link::test::HandDrivenLink;
use std::time::Duration;

impl HandDrivenLink {
    pub(crate) fn turn_at(&mut self, at: Duration) {
        note_turn(&self.shared, &mut self.last_turn, self.start + at);
    }
}
