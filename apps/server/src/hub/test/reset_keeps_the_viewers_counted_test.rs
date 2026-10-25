use crate::fixtures::hub_with_config;

#[test]
fn reset_keeps_the_viewers_counted() {
    let hub = hub_with_config();
    let _subscription = hub.join();

    hub.reset();

    assert_eq!(hub.viewer_count(), 1);
}
