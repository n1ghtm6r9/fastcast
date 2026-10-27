use crate::hub::Hub;

#[test]
fn an_ask_made_before_anyone_watches_is_not_kept() {
    let hub = Hub::new();

    hub.ask_scale(1);
    let mut watcher = hub.watch_scale_asks();
    hub.ask_scale(-1);

    assert_eq!(watcher.try_recv(), Ok(-1));
}
