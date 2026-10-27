use super::scale_at;
use crate::hub::Hub;

#[test]
fn the_same_scale_shown_again_wakes_the_watchers_again() {
    let hub = Hub::new();
    let mut watcher = hub.watch_scale();

    hub.show_scale(Some(scale_at(150)));
    watcher.mark_unchanged();
    hub.show_scale(Some(scale_at(150)));

    assert!(watcher.has_changed().expect("the hub is alive"));
}
