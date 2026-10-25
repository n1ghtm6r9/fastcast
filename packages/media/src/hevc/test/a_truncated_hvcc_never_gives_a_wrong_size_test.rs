use super::{HevcSpsParts, hevc_nal_unit, hvcc_with_arrays};
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn a_truncated_hvcc_never_gives_a_wrong_size() {
    let vps = hevc_nal_unit(32, &[1, 2, 3]);
    let sps = HevcSpsParts::default().nal_unit();
    let pps = hevc_nal_unit(34, &[9]);
    let hvcc = hvcc_with_arrays(&[
        (32, vec![vps.clone()]),
        (33, vec![sps.clone()]),
        (34, vec![pps]),
    ]);
    let sps_end = 23 + (3 + 2 + vps.len()) + (3 + 2 + sps.len());

    for length in 0..=hvcc.len() {
        let size = video_size_from_hvcc(&hvcc[..length]);

        if length < sps_end {
            assert_eq!(size, None, "{length} bytes");
        } else {
            assert_eq!(size, Some(video_size(1920, 1080)), "{length} bytes");
        }
    }
}
