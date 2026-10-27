use super::levels_of;
use crate::display::display_scale_of;

#[test]
fn levels_past_the_known_percents_have_no_scale() {
    assert_eq!(display_scale_of(levels_of(-2, 0, 10)), None);
}
