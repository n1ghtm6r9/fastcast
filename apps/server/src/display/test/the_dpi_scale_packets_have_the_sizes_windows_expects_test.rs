use crate::display::{DpiScaleChange, DpiScaleRequest};

#[test]
fn the_dpi_scale_packets_have_the_sizes_windows_expects() {
    assert_eq!(size_of::<DpiScaleRequest>(), 0x20);
    assert_eq!(size_of::<DpiScaleChange>(), 0x18);
}
