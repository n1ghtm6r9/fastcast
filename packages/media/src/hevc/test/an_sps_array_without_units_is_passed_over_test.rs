use super::{HevcSpsParts, hvcc_with_arrays};
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn an_sps_array_without_units_is_passed_over() {
    let hvcc = hvcc_with_arrays(&[
        (33, Vec::new()),
        (33, vec![HevcSpsParts::default().nal_unit()]),
    ]);

    assert_eq!(video_size_from_hvcc(&hvcc), Some(video_size(1920, 1080)));
}
