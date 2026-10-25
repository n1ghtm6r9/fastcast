use super::{real_avcc_record_named, video_size};
use crate::video_size_from_avcc;

#[test]
fn a_constrained_baseline_stream_has_no_chroma_fields() {
    let record = real_avcc_record_named("x264_constrained_baseline_1366x768");

    assert_eq!(
        video_size_from_avcc(&record.avcc),
        Some(video_size(1366, 768))
    );
}
