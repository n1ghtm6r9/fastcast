use crate::fixtures::{config_of, hub_with_config, nvenc_2560x1440_avcc};

#[test]
fn the_same_config_set_again_is_not_a_change() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);

    assert!(!hub.set_config(config_of(&nvenc_2560x1440_avcc(), 60)));
    assert_eq!(hub.join().gop.len(), 1);
}
