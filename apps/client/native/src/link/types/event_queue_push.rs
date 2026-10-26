use crate::link::{EventQueue, LinkEvent};

impl EventQueue {
    pub fn push(&self, event: LinkEvent) {
        let mut state = self.lock();

        if event.is_frame() {
            state.frames += 1;
        }

        state.events.push_back(event);
        drop(state);
        self.ready.notify_one();
    }
}
