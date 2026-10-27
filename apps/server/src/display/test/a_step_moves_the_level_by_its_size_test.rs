use super::levels_of;
use crate::display::stepped_scale_level;

#[test]
fn a_step_moves_the_level_by_its_size() {
    assert_eq!(stepped_scale_level(levels_of(-2, 0, 4), 1), 1);
    assert_eq!(stepped_scale_level(levels_of(-2, 0, 4), -2), -2);
}
