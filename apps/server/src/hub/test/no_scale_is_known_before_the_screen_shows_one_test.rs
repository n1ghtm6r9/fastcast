use crate::hub::Hub;

#[test]
fn no_scale_is_known_before_the_screen_shows_one() {
    assert_eq!(*Hub::new().watch_scale().borrow(), None);
}
