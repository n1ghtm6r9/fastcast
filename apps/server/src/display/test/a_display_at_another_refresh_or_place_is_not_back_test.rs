use super::{mode_of, placement_of};
use crate::display::layout_is_back;
use std::slice;

#[test]
fn a_display_at_another_refresh_or_place_is_not_back() {
    let saved = placement_of("\\\\.\\DISPLAY1", 0, 0, mode_of(2560, 1440, 200));
    let slower = placement_of("\\\\.\\DISPLAY1", 0, 0, mode_of(2560, 1440, 60));
    let moved = placement_of("\\\\.\\DISPLAY1", 3840, 0, mode_of(2560, 1440, 200));

    assert!(!layout_is_back(slice::from_ref(&saved), &[slower]));
    assert!(!layout_is_back(slice::from_ref(&saved), &[moved]));
}
