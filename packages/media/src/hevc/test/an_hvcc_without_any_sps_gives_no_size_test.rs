use super::{hevc_nal_unit, hvcc_with_arrays};
use crate::video_size_from_hvcc;

#[test]
fn an_hvcc_without_any_sps_gives_no_size() {
    let hvcc = hvcc_with_arrays(&[
        (32, vec![hevc_nal_unit(32, &[1, 2, 3])]),
        (34, vec![hevc_nal_unit(34, &[9])]),
    ]);

    assert_eq!(video_size_from_hvcc(&hvcc), None);
}
