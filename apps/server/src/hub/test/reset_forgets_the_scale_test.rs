use super::scale_at;
use crate::hub::Hub;

#[test]
fn reset_forgets_the_scale() {
    let hub = Hub::new();

    hub.show_scale(Some(scale_at(150)));
    hub.reset();

    assert_eq!(*hub.watch_scale().borrow(), None);
}
