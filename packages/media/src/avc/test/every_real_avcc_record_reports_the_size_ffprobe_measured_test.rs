use super::{real_avcc_records, video_size};
use crate::video_size_from_avcc;

#[test]
fn every_real_avcc_record_reports_the_size_ffprobe_measured() {
    let records = real_avcc_records();

    assert_eq!(records.len(), 16);

    for record in records {
        assert_eq!(
            video_size_from_avcc(&record.avcc),
            Some(video_size(record.width, record.height)),
            "{}",
            record.name
        );
    }
}
