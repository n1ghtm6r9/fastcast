use crate::capture::Picture;

#[test]
fn a_converted_picture_is_not_converted_again() {
    let mut picture = Picture::default();

    picture.arrived();
    picture.take_conversion();

    assert!(picture.exists());
    assert!(!picture.take_conversion());
}
