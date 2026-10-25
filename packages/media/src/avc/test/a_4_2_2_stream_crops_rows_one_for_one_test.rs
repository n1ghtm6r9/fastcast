use super::{real_avcc_record_named, video_size};
use crate::video_size_from_avcc;

#[test]
fn a_4_2_2_stream_crops_rows_one_for_one() {
    let record = real_avcc_record_named("x264_high422_1920x1080");

    assert_eq!(
        video_size_from_avcc(&record.avcc),
        Some(video_size(1920, 1080))
    );
}
