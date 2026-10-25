use super::{real_avcc_record_named, video_size};
use crate::video_size_from_avcc;

#[test]
fn a_4_4_4_stream_crops_columns_one_for_one() {
    let record = real_avcc_record_named("x264_high444_1366x768");

    assert_eq!(
        video_size_from_avcc(&record.avcc),
        Some(video_size(1366, 768))
    );
}
