use crate::receiver::{FrameReceiver, MAX_PENDING_FRAMES};

impl FrameReceiver {
    pub(crate) fn enforce_pending_limit(&mut self) {
        let Some(&highest) = self.pending.keys().next_back() else {
            return;
        };
        let limit = MAX_PENDING_FRAMES as u64;

        if highest + 1 - self.expected > limit {
            self.declare_lost(highest + 1 - limit);
        }
    }
}
