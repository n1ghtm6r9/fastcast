use super::levels_of;
use crate::display::display_scale_of;

#[test]
fn a_current_level_past_the_highest_reads_as_the_highest() {
    let scale = display_scale_of(levels_of(-2, 9, 4)).expect("the levels are known percents");

    assert_eq!(scale.percent, 250);
}
