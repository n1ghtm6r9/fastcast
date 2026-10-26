use crate::hub::StreamConfig;
use std::sync::Arc;

pub struct SessionCursor {
    pub(crate) sent_config: Option<Arc<StreamConfig>>,
    pub(crate) waiting_for_key_frame: bool,
}

impl SessionCursor {
    pub fn starting_with(sent_config: Option<Arc<StreamConfig>>) -> Self {
        Self {
            sent_config,
            waiting_for_key_frame: true,
        }
    }
}
