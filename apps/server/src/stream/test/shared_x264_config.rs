use crate::fixtures::{config_of, x264_1920x1080_avcc};
use crate::hub::StreamConfig;
use std::sync::Arc;

pub fn shared_x264_config() -> Arc<StreamConfig> {
    Arc::new(config_of(&x264_1920x1080_avcc(), 60))
}
