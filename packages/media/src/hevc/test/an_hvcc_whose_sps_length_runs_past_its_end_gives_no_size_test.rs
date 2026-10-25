use super::{HevcSpsParts, hvcc_with_arrays};
use crate::video_size_from_hvcc;

#[test]
fn an_hvcc_whose_sps_length_runs_past_its_end_gives_no_size() {
    let mut hvcc = hvcc_with_arrays(&[(33, vec![HevcSpsParts::default().nal_unit()])]);

    hvcc.truncate(hvcc.len() - 4);

    assert_eq!(video_size_from_hvcc(&hvcc), None);
}
