use crate::fixtures::{config_of, hub_with_config, nvenc_2560x1440_avcc};

#[test]
fn a_config_with_another_frame_rate_is_a_change() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);

    assert!(hub.set_config(config_of(&nvenc_2560x1440_avcc(), 30)));

    let subscription = hub.join();

    assert!(subscription.gop.is_empty());
    assert_eq!(subscription.config.expect("the config is stored").fps, 30);
}
