use super::{real_avcc_record_named, video_size};
use crate::video_size_from_avcc;

#[test]
fn a_1366_wide_stream_is_cropped_from_1376_coded_columns() {
    let record = real_avcc_record_named("nvenc_high_1366x768");

    assert_eq!(
        video_size_from_avcc(&record.avcc),
        Some(video_size(1366, 768))
    );
}
