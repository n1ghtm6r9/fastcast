use super::mode_of;

#[test]
fn a_mode_reads_as_width_x_height_at_refresh() {
    assert_eq!(mode_of(3840, 2160, 60).to_string(), "3840x2160@60");
}
