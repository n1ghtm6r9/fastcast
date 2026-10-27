use super::levels_of;
use crate::display::display_scale_of;

#[test]
fn each_level_above_zero_is_one_step_up_the_percent_list() {
    let scale = display_scale_of(levels_of(-2, 3, 4)).expect("the levels are known percents");

    assert_eq!(scale.percent, 225);
}
