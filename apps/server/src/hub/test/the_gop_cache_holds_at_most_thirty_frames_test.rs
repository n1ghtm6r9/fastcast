use crate::fixtures::hub_with_config;

#[test]
fn the_gop_cache_holds_at_most_thirty_frames() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);

    for index in 1..30 {
        hub.publish(false, index, &[2]);
    }

    assert_eq!(hub.join().gop.len(), 30);

    hub.publish(false, 30, &[2]);

    assert!(hub.join().gop.is_empty());
}
