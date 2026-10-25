use crate::display::DisplayBounds;

#[test]
fn the_edge_of_a_display_never_overflows() {
    let far = DisplayBounds {
        x: i32::MAX - 1,
        y: 0,
        width: u32::MAX,
        height: 1080,
    };

    assert_eq!(far.right_edge(), i32::MAX);
}
