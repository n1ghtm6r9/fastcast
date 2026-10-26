use crate::receiver::{FrameReceiver, MAX_PENDING_BYTES};

impl FrameReceiver {
    pub(crate) fn enforce_pending_bytes(&mut self) {
        while self.pending_bytes > MAX_PENDING_BYTES && self.pending.len() > 1 {
            let Some((&oldest, _)) = self.pending.first_key_value() else {
                return;
            };

            self.declare_lost(oldest + 1);
        }
    }
}
