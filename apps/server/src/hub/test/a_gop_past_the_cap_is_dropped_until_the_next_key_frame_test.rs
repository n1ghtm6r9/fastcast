use super::timestamps_of;
use crate::fixtures::hub_with_config;
use crate::hub::MAX_GOP_FRAMES;

#[test]
fn a_gop_past_the_cap_is_dropped_until_the_next_key_frame() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);

    for index in 1..=MAX_GOP_FRAMES {
        hub.publish(false, index as u32, &[2]);
    }

    assert!(hub.join().gop.is_empty());

    hub.publish(false, 9_000, &[3]);

    assert!(hub.join().gop.is_empty());

    hub.publish(true, 9_001, &[4]);

    assert_eq!(timestamps_of(&hub.join().gop), vec![9_001]);
}
