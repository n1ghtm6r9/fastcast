use crate::hub::Frame;
use crate::web::{Admission, ViewerCursor};
use std::sync::Arc;

impl ViewerCursor {
    pub fn admit(&mut self, frame: &Frame) -> Admission {
        let config_already_sent = self
            .sent_config
            .as_ref()
            .is_some_and(|sent| Arc::ptr_eq(sent, &frame.config));

        if !frame.key {
            return if self.waiting_for_key_frame || !config_already_sent {
                Admission::Skip
            } else {
                Admission::Send
            };
        }

        self.waiting_for_key_frame = false;

        if config_already_sent {
            return Admission::Send;
        }

        self.sent_config = Some(frame.config.clone());

        Admission::SendWithConfig(frame.config.clone())
    }
}
