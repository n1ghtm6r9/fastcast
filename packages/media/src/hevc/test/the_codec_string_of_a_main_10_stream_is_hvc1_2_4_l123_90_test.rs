use super::real_hvcc_record_named;
use crate::hevc_codec_string;

#[test]
fn the_codec_string_of_a_main_10_stream_is_hvc1_2_4_l123_90() {
    let record = real_hvcc_record_named("x265_main10_1920x1080");

    assert_eq!(
        hevc_codec_string(&record.hvcc),
        Some("hvc1.2.4.L123.90".to_string())
    );
}
