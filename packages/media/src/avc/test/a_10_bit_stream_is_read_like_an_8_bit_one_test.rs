use super::{real_avcc_record_named, video_size};
use crate::video_size_from_avcc;

#[test]
fn a_10_bit_stream_is_read_like_an_8_bit_one() {
    let record = real_avcc_record_named("x264_high10_1280x720");

    assert_eq!(
        video_size_from_avcc(&record.avcc),
        Some(video_size(1280, 720))
    );
}
