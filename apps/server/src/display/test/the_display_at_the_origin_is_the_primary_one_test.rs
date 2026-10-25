use super::{mode_of, placement_of};

#[test]
fn the_display_at_the_origin_is_the_primary_one() {
    assert!(placement_of("\\\\.\\DISPLAY1", 0, 0, mode_of(2560, 1440, 200)).is_primary());
    assert!(!placement_of("\\\\.\\DISPLAY2", 2560, 0, mode_of(1920, 1080, 60)).is_primary());
    assert!(!placement_of("\\\\.\\DISPLAY3", 0, -1080, mode_of(1920, 1080, 60)).is_primary());
}
