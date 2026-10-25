use super::real_hvcc_record_named;
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn a_monochrome_stream_crops_in_single_pixel_units() {
    let record = real_hvcc_record_named("x265_monochrome_1366x768");

    assert_eq!(
        video_size_from_hvcc(&record.hvcc),
        Some(video_size(1366, 768))
    );
}
