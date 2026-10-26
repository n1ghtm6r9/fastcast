use crate::fixtures::{config_of, x264_1920x1080_avcc};
use crate::hub::Hub;
use std::sync::Arc;

pub fn hub_with_x264_config() -> Arc<Hub> {
    let hub = Hub::new();

    hub.set_config(config_of(&x264_1920x1080_avcc(), 60));

    Arc::new(hub)
}
