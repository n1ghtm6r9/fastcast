use crate::app::{FASTCAST_ICON, largest_icon_picture};

#[test]
fn a_cut_icon_has_no_largest_picture() {
    assert_eq!(largest_icon_picture(&FASTCAST_ICON[..200]), None);
    assert_eq!(largest_icon_picture(&FASTCAST_ICON[..20]), None);
}
