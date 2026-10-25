use super::another_request;
use crate::fixtures::hub_with_config;

#[test]
fn a_joiner_asking_for_another_picture_gets_no_snapshot_of_the_old_stream() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);
    hub.publish(false, 1, &[2]);

    assert!(hub.request(another_request()));

    let subscription = hub.join();

    assert!(subscription.config.is_none());
    assert!(subscription.gop.is_empty());
}
