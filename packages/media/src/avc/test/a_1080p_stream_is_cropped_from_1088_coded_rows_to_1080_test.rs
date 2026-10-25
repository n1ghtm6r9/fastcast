use super::{real_avcc_record_named, video_size};
use crate::video_size_from_avcc;

#[test]
fn a_1080p_stream_is_cropped_from_1088_coded_rows_to_1080() {
    let record = real_avcc_record_named("nvenc_high_1920x1080");

    assert_eq!(
        video_size_from_avcc(&record.avcc),
        Some(video_size(1920, 1080))
    );
}
