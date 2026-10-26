use crate::link::{LinkEvent, QueueState};

impl QueueState {
    pub(crate) fn drop_frames_except_the_latest_config(&mut self) {
        let latest_config = self
            .events
            .iter()
            .rposition(|event| matches!(event, LinkEvent::Config(_)));
        let mut position = 0;

        self.events.retain(|event| {
            let kept = !event.is_frame() || Some(position) == latest_config;
            position += 1;
            kept
        });
        self.frames = usize::from(latest_config.is_some());
    }
}
