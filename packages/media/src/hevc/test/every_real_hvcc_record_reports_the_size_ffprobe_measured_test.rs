use super::real_hvcc_records;
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn every_real_hvcc_record_reports_the_size_ffprobe_measured() {
    let records = real_hvcc_records();

    assert_eq!(records.len(), 22);

    for record in records {
        assert_eq!(
            video_size_from_hvcc(&record.hvcc),
            Some(video_size(record.width, record.height)),
            "{}",
            record.name
        );
    }
}
