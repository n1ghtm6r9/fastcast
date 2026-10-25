use super::timestamps_of;
use crate::fixtures::hub_with_config;

#[test]
fn delta_frames_before_the_first_key_frame_are_not_kept() {
    let hub = hub_with_config();

    hub.publish(false, 0, &[1]);
    hub.publish(false, 1, &[2]);

    assert!(hub.join().gop.is_empty());

    hub.publish(true, 2, &[3]);

    assert_eq!(timestamps_of(&hub.join().gop), vec![2]);
}
