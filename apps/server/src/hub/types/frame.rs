use crate::hub::StreamConfig;
use bytes::Bytes;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug)]
pub struct Frame {
    pub key: bool,
    pub wire: Bytes,
    pub config: Arc<StreamConfig>,
    pub shown: Option<Instant>,
}
