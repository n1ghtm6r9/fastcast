use super::real_av1c_records;
use crate::avc::test::video_size;
use crate::video_size_from_av1c;

#[test]
fn every_real_av1c_record_reports_the_size_ffprobe_measured() {
    for record in real_av1c_records() {
        assert_eq!(
            video_size_from_av1c(&record.av1c),
            Some(video_size(record.width, record.height)),
            "{}",
            record.name
        );
    }
}
