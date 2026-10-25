use super::{mode_of, placement_of};
use crate::display::layout_is_back;
use std::slice;

#[test]
fn the_layout_is_back_when_every_saved_display_is_in_place() {
    let monitor = placement_of("\\\\.\\DISPLAY1", 0, 0, mode_of(2560, 1440, 200));
    let side = placement_of("\\\\.\\DISPLAY2", 2560, 0, mode_of(1920, 1080, 60));
    let leftover = placement_of("\\\\.\\DISPLAY10", 4480, 0, mode_of(3840, 2160, 60));

    assert!(layout_is_back(
        &[monitor.clone(), side.clone()],
        &[side, monitor.clone()]
    ));
    assert!(layout_is_back(
        slice::from_ref(&monitor),
        &[monitor.clone(), leftover]
    ));
    assert!(layout_is_back(&[], &[]));
}
