use crate::app::window_icon;

#[test]
fn the_window_icon_is_the_largest_picture_of_the_fastcast_icon() {
    let icon = window_icon().expect("the fastcast icon has a picture");

    assert_eq!((icon.width, icon.height), (256, 256));
    assert_eq!(icon.rgba.len(), 256 * 256 * 4);
}
