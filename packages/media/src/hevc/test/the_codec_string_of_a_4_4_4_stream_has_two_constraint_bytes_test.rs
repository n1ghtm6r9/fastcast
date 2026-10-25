use super::real_hvcc_record_named;
use crate::hevc_codec_string;

#[test]
fn the_codec_string_of_a_4_4_4_stream_has_two_constraint_bytes() {
    let record = real_hvcc_record_named("x265_main444_1920x1080");

    assert_eq!(
        hevc_codec_string(&record.hvcc),
        Some("hvc1.4.10.L123.9E.08".to_string())
    );
}
