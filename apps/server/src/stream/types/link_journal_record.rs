use crate::stream::{LINK_JOURNAL_INTERVAL, LinkJournal, link_line};
use std::time::Instant;
use wire::Feedback;

impl LinkJournal {
    pub fn record(&mut self, feedback: &Feedback, fec_percent: u8, now: Instant) -> Option<String> {
        if feedback.keyframe_needed && !self.asking {
            self.keyframe_asks += 1;
        }
        self.asking = feedback.keyframe_needed;

        let Some((since, first)) = self.since else {
            self.since = Some((now, *feedback));

            return None;
        };

        if now.saturating_duration_since(since) < LINK_JOURNAL_INTERVAL {
            return None;
        }

        let line = link_line(&first, feedback, self.keyframe_asks, fec_percent);
        self.since = Some((now, *feedback));
        self.keyframe_asks = 0;

        Some(line)
    }
}
