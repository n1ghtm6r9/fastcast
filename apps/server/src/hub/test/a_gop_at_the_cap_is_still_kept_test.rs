use crate::fixtures::hub_with_config;
use crate::hub::MAX_GOP_FRAMES;

#[test]
fn a_gop_at_the_cap_is_still_kept() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);

    for index in 1..MAX_GOP_FRAMES {
        hub.publish(false, index as u32, &[2]);
    }

    assert_eq!(hub.join().gop.len(), MAX_GOP_FRAMES);
}
