use crate::receiver::{FRAME_TIMEOUT, FrameReceiver, LossDeadline, REORDER_WINDOW};

impl FrameReceiver {
    pub(crate) fn loss_after(&self, gap: u64) -> Option<LossDeadline> {
        let timeout = self
            .pending
            .range(gap..)
            .map(|(_, frame)| frame.first_seen)
            .min()
            .map(|first_seen| LossDeadline {
                at: first_seen + FRAME_TIMEOUT,
                end: gap + 1,
            });
        let reorder = self.pending.range(gap + 1..).find_map(|(&number, frame)| {
            frame.completed_at.map(|completed_at| LossDeadline {
                at: completed_at + REORDER_WINDOW,
                end: number,
            })
        });

        [timeout, reorder]
            .into_iter()
            .flatten()
            .min_by_key(|deadline| deadline.at)
    }
}
