use crate::gpu::desktop_extent;

#[test]
fn the_extent_of_a_rectangle_spanning_the_origin_is_its_full_length() {
    assert_eq!(desktop_extent(-1920, 0), 1920);
    assert_eq!(desktop_extent(-960, 1920), 2880);
}
