use crate::capture::Picture;

#[test]
fn no_picture_exists_before_the_first_one_arrives() {
    let mut picture = Picture::default();

    assert!(!picture.exists());
    assert!(!picture.take_conversion());
}
