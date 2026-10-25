use super::real_hvcc_record_named;
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn a_4k_stream_is_cropped_from_2176_coded_rows_to_2160() {
    let record = real_hvcc_record_named("nvenc_main_3840x2160");

    assert_eq!(
        video_size_from_hvcc(&record.hvcc),
        Some(video_size(3840, 2160))
    );
}
