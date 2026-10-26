use crate::hub::Hub;

#[tokio::test]
async fn a_new_report_replaces_the_last_one_and_wakes_the_watchers() {
    let hub = Hub::new();
    let mut watcher = hub.watch_reports();

    hub.report("{\"decoded\":60}".to_owned());
    hub.report("{\"decoded\":120}".to_owned());

    watcher.changed().await.expect("the hub is alive");

    assert_eq!(watcher.borrow().as_deref(), Some("{\"decoded\":120}"));
}
