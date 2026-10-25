use super::real_hvcc_record_named;
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn a_4_4_4_stream_crops_columns_one_for_one() {
    let record = real_hvcc_record_named("nvenc_rext444_1366x768");

    assert_eq!(
        video_size_from_hvcc(&record.hvcc),
        Some(video_size(1366, 768))
    );
}
