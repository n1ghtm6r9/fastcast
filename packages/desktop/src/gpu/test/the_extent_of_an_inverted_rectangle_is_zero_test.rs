use crate::gpu::desktop_extent;

#[test]
fn the_extent_of_an_inverted_rectangle_is_zero() {
    assert_eq!(desktop_extent(5, 3), 0);
}
