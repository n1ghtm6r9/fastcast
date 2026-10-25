use super::timestamps_of;
use crate::fixtures::hub_with_config;

#[test]
fn a_late_joiner_gets_the_config_and_the_frames_since_the_last_key_frame() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);
    hub.publish(false, 1, &[2]);
    hub.publish(false, 2, &[3]);
    hub.publish(true, 3, &[4]);
    hub.publish(false, 4, &[5]);

    let subscription = hub.join();

    assert!(subscription.config.is_some());
    assert_eq!(timestamps_of(&subscription.gop), vec![3, 4]);
    assert!(subscription.gop[0].key);
    assert!(!subscription.gop[1].key);
}
