use crate::fixtures::{config_of, nvenc_2560x1440_avcc};
use crate::hub::Hub;

#[test]
fn a_new_config_is_stored_and_reported_as_a_change() {
    let hub = Hub::new();
    let config = config_of(&nvenc_2560x1440_avcc(), 60);
    let json = config.json.clone();

    assert!(hub.set_config(config));
    assert_eq!(hub.join().config.expect("the config is stored").json, json);
}
