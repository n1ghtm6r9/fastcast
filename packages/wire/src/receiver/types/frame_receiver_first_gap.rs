use crate::receiver::{FrameReceiver, PendingFrame};

impl FrameReceiver {
    pub(crate) fn first_gap(&self) -> Option<u64> {
        let highest = *self.pending.keys().next_back()?;
        let mut gap = self.expected;

        while self
            .pending
            .get(&gap)
            .is_some_and(PendingFrame::is_complete)
        {
            gap += 1;
        }

        (gap <= highest).then_some(gap)
    }
}
