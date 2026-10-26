use crate::hub::StreamConfig;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Admission {
    Skip,
    Send,
    SendWithConfig(Arc<StreamConfig>),
}
