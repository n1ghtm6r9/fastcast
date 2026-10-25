use crate::fixtures::{config_of, hub_with_config, x264_1920x1080_avcc};

#[test]
fn a_config_with_another_stream_clears_the_gop() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);
    hub.publish(false, 1, &[2]);

    assert!(hub.set_config(config_of(&x264_1920x1080_avcc(), 60)));

    let subscription = hub.join();
    let config = subscription.config.expect("the config is stored");

    assert!(subscription.gop.is_empty());
    assert_eq!((config.width, config.height), (1920, 1080));
}
