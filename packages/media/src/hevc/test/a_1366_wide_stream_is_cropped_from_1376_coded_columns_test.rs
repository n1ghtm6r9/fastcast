use super::real_hvcc_record_named;
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn a_1366_wide_stream_is_cropped_from_1376_coded_columns() {
    let record = real_hvcc_record_named("nvenc_main_1366x768");

    assert_eq!(
        video_size_from_hvcc(&record.hvcc),
        Some(video_size(1366, 768))
    );
}
