use crate::hub::{Frame, StreamConfig};
use bytes::Bytes;
use std::sync::Arc;

pub fn frame_under(config: &Arc<StreamConfig>, key: bool) -> Frame {
    Frame {
        key,
        wire: Bytes::from_static(&[1, 2, 3]),
        config: config.clone(),
        shown: None,
    }
}
