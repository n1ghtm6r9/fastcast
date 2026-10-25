use super::{SpsParts, avcc_with_sps_units};
use crate::video_size_from_avcc;

#[test]
fn an_avcc_whose_sps_length_runs_past_its_end_gives_no_size() {
    let mut avcc = avcc_with_sps_units(&[SpsParts::default().nal_unit()]);

    avcc.truncate(avcc.len() - 4);

    assert_eq!(video_size_from_avcc(&avcc), None);
}
