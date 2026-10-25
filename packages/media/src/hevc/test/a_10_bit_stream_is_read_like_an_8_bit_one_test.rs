use super::real_hvcc_record_named;
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn a_10_bit_stream_is_read_like_an_8_bit_one() {
    let record = real_hvcc_record_named("nvenc_main10_1920x1080");

    assert_eq!(
        video_size_from_hvcc(&record.hvcc),
        Some(video_size(1920, 1080))
    );
}
