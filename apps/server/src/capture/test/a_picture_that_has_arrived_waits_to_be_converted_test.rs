use crate::capture::Picture;

#[test]
fn a_picture_that_has_arrived_waits_to_be_converted() {
    let mut picture = Picture::default();

    picture.arrived();

    assert!(picture.exists());
    assert!(picture.take_conversion());
}
