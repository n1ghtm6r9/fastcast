use crate::hub::{Hub, ViewerRequest};

impl Hub {
    pub fn request(&self, request: ViewerRequest) -> bool {
        let mut state = self.lock_state();

        let changed = self.requests.send_if_modified(|current| {
            if *current == request {
                return false;
            }

            *current = request;

            true
        });

        if changed && state.config.is_some() {
            state.config = None;
            state.gop.clear();
            state.awaiting_reset = true;
        }

        changed
    }
}
