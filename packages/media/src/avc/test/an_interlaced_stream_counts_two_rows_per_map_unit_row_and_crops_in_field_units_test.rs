use super::{real_avcc_record_named, video_size};
use crate::video_size_from_avcc;

#[test]
fn an_interlaced_stream_counts_two_rows_per_map_unit_row_and_crops_in_field_units() {
    let record = real_avcc_record_named("x264_main_interlaced_1920x1080");

    assert_eq!(
        video_size_from_avcc(&record.avcc),
        Some(video_size(1920, 1080))
    );
}
