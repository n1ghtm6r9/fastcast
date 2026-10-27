use crate::hub::{Hub, Subscription, ViewerGuard};

impl Hub {
    pub fn join(&self) -> Subscription {
        let state = self.lock_state();

        Subscription {
            config: state.config.clone(),
            gop: state.gop.clone(),
            frames: self.frames.subscribe(),
            scales: self.scale.subscribe(),
            diagnose: *self.diagnose.borrow(),
            guard: ViewerGuard::new(self.viewers.clone()),
        }
    }
}
