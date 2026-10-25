use crate::display::{DisplayBounds, rightmost_edge};

#[test]
fn a_display_left_of_the_origin_does_not_move_the_edge() {
    let left = DisplayBounds {
        x: -1920,
        y: 0,
        width: 1920,
        height: 1080,
    };
    let primary = DisplayBounds {
        x: 0,
        y: 0,
        width: 2560,
        height: 1440,
    };

    assert_eq!(rightmost_edge(&[left, primary]), 2560);
    assert_eq!(rightmost_edge(&[left]), 0);
}
