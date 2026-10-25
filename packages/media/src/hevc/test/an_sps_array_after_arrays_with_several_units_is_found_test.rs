use super::{HevcSpsParts, hevc_nal_unit, hvcc_with_arrays};
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn an_sps_array_after_arrays_with_several_units_is_found() {
    let hvcc = hvcc_with_arrays(&[
        (
            32,
            vec![hevc_nal_unit(32, &[1, 2, 3]), hevc_nal_unit(32, &[4, 5])],
        ),
        (33, vec![HevcSpsParts::default().nal_unit()]),
        (34, vec![hevc_nal_unit(34, &[9])]),
    ]);

    assert_eq!(video_size_from_hvcc(&hvcc), Some(video_size(1920, 1080)));
}
