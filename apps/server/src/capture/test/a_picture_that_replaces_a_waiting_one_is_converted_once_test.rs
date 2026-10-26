use crate::capture::Picture;

#[test]
fn a_picture_that_replaces_a_waiting_one_is_converted_once() {
    let mut picture = Picture::default();

    picture.arrived();
    picture.arrived();

    assert!(picture.take_conversion());
    assert!(!picture.take_conversion());
}
