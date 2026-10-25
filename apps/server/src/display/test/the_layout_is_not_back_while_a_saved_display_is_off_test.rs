use super::{mode_of, placement_of};
use crate::display::layout_is_back;

#[test]
fn the_layout_is_not_back_while_a_saved_display_is_off() {
    let monitor = placement_of("\\\\.\\DISPLAY1", 0, 0, mode_of(2560, 1440, 200));
    let virtual_display = placement_of("\\\\.\\DISPLAY10", 0, 0, mode_of(3840, 2160, 60));

    assert!(!layout_is_back(&[monitor], &[virtual_display]));
}
