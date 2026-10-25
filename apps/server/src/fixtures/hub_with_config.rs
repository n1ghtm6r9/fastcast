use crate::fixtures::{config_of, nvenc_2560x1440_avcc};
use crate::hub::Hub;

pub fn hub_with_config() -> Hub {
    let hub = Hub::new();

    hub.set_config(config_of(&nvenc_2560x1440_avcc(), 60));

    hub
}
