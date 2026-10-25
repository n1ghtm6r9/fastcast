use super::{real_avcc_record_named, video_size};
use crate::video_size_from_avcc;

#[test]
fn an_nvenc_stream_at_2560x1440_reports_2560x1440() {
    let record = real_avcc_record_named("nvenc_high_2560x1440");

    assert_eq!(
        video_size_from_avcc(&record.avcc),
        Some(video_size(2560, 1440))
    );
}
