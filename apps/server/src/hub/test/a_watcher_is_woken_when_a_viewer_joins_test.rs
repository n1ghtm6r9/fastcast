use crate::hub::Hub;

#[tokio::test]
async fn a_watcher_is_woken_when_a_viewer_joins() {
    let hub = Hub::new();
    let mut watcher = hub.watch_viewers();
    let _subscription = hub.join();

    watcher.changed().await.expect("the hub is alive");

    assert_eq!(*watcher.borrow(), 1);
}
