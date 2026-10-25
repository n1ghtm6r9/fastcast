use super::real_hvcc_record_named;
use crate::avc::test::video_size;
use crate::video_size_from_hvcc;

#[test]
fn a_stream_with_two_temporal_layers_skips_the_sub_layer_fields() {
    let record = real_hvcc_record_named("x265_two_temporal_layers_1920x1080");

    assert_eq!(
        video_size_from_hvcc(&record.hvcc),
        Some(video_size(1920, 1080))
    );
}
