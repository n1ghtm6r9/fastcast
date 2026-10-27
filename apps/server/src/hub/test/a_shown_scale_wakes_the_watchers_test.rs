use super::scale_at;
use crate::hub::Hub;

#[test]
fn a_shown_scale_wakes_the_watchers() {
    let hub = Hub::new();
    let mut watcher = hub.watch_scale();

    hub.show_scale(Some(scale_at(175)));

    assert!(watcher.has_changed().expect("the hub is alive"));
    assert_eq!(*watcher.borrow_and_update(), Some(scale_at(175)));
}
