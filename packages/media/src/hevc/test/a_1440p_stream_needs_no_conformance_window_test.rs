use super::real_hvcc_record_named;
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn a_1440p_stream_needs_no_conformance_window() {
    let record = real_hvcc_record_named("nvenc_main_2560x1440");

    assert_eq!(
        video_size_from_hvcc(&record.hvcc),
        Some(video_size(2560, 1440))
    );
}
