use crate::display::{DisplayBounds, rightmost_edge};

#[test]
fn the_new_display_goes_right_of_the_rightmost_display() {
    let physical = DisplayBounds {
        x: 0,
        y: 0,
        width: 2560,
        height: 1440,
    };
    let beside = DisplayBounds {
        x: 2560,
        y: 0,
        width: 1920,
        height: 1080,
    };

    assert_eq!(rightmost_edge(&[physical]), 2560);
    assert_eq!(rightmost_edge(&[physical, beside]), 4480);
    assert_eq!(rightmost_edge(&[beside, physical]), 4480);
}
