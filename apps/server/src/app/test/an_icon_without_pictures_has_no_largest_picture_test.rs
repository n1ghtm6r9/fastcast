use crate::app::largest_icon_picture;

#[test]
fn an_icon_without_pictures_has_no_largest_picture() {
    assert_eq!(largest_icon_picture(&[0, 0, 1, 0, 0, 0]), None);
    assert_eq!(largest_icon_picture(&[]), None);
}
