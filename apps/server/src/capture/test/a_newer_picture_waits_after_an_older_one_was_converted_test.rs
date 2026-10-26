use crate::capture::Picture;

#[test]
fn a_newer_picture_waits_after_an_older_one_was_converted() {
    let mut picture = Picture::default();

    picture.arrived();
    picture.take_conversion();
    picture.arrived();

    assert!(picture.exists());
    assert!(picture.take_conversion());
}
