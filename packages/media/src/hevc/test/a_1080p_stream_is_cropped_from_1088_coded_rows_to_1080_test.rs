use super::real_hvcc_record_named;
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn a_1080p_stream_is_cropped_from_1088_coded_rows_to_1080() {
    let record = real_hvcc_record_named("nvenc_main_1920x1080");

    assert_eq!(
        video_size_from_hvcc(&record.hvcc),
        Some(video_size(1920, 1080))
    );
}
