use super::levels_of;
use crate::display::stepped_scale_level;

#[test]
fn a_step_never_leaves_the_levels_the_screen_offers() {
    assert_eq!(stepped_scale_level(levels_of(-2, 4, 4), 1), 4);
    assert_eq!(stepped_scale_level(levels_of(-2, -2, 4), -1), -2);
    assert_eq!(stepped_scale_level(levels_of(-2, 0, 4), i32::MAX), 4);
}
