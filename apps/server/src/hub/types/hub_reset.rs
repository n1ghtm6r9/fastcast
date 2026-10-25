use crate::hub::Hub;

impl Hub {
    pub fn reset(&self) {
        let mut state = self.lock_state();

        state.config = None;
        state.gop.clear();
        state.awaiting_reset = false;
    }
}
