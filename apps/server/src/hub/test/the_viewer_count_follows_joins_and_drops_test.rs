use crate::hub::Hub;

#[test]
fn the_viewer_count_follows_joins_and_drops() {
    let hub = Hub::new();

    assert_eq!(hub.viewer_count(), 0);

    let first = hub.join();

    assert_eq!(hub.viewer_count(), 1);

    let second = hub.join();

    assert_eq!(hub.viewer_count(), 2);

    drop(first);

    assert_eq!(hub.viewer_count(), 1);

    drop(second);

    assert_eq!(hub.viewer_count(), 0);
}
