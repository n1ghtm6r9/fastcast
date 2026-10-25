use crate::fixtures::{config_of, hub_with_config, x264_1920x1080_avcc};
use std::sync::Arc;

#[test]
fn frames_carry_the_config_they_were_published_under() {
    let hub = hub_with_config();
    let mut subscription = hub.join();

    hub.publish(true, 0, &[1]);
    hub.set_config(config_of(&x264_1920x1080_avcc(), 60));
    hub.publish(true, 1, &[2]);

    let first = subscription.frames.try_recv().expect("the first frame");
    let second = subscription.frames.try_recv().expect("the second frame");
    let current = hub.join().config.expect("the config is stored");

    assert_eq!((first.config.width, first.config.height), (2560, 1440));
    assert_eq!((second.config.width, second.config.height), (1920, 1080));
    assert!(Arc::ptr_eq(&second.config, &current));
    assert!(!Arc::ptr_eq(&first.config, &current));
}
