use crate::fixtures::{config_of, nvenc_2560x1440_avcc};
use crate::hub::StreamConfig;
use std::sync::Arc;

pub fn shared_nvenc_config() -> Arc<StreamConfig> {
    Arc::new(config_of(&nvenc_2560x1440_avcc(), 60))
}
