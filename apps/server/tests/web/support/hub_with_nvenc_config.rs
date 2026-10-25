use crate::support::{config_from, nvenc_avcc};
use app::hub::Hub;
use std::sync::Arc;

pub fn hub_with_nvenc_config() -> Arc<Hub> {
    let hub = Hub::new();

    hub.set_config(config_from(&nvenc_avcc(), 60));

    Arc::new(hub)
}
