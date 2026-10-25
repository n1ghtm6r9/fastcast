use crate::hub::{Hub, StreamConfig};
use std::sync::Arc;

impl Hub {
    pub fn set_config(&self, config: StreamConfig) -> bool {
        let mut state = self.lock_state();

        if state.awaiting_reset {
            return false;
        }

        let unchanged = state
            .config
            .as_deref()
            .is_some_and(|current| *current == config);

        if unchanged {
            return false;
        }

        state.config = Some(Arc::new(config));
        state.gop.clear();

        true
    }
}
