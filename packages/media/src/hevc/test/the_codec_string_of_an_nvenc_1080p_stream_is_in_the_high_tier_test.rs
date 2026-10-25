use super::real_hvcc_record_named;
use crate::hevc_codec_string;

#[test]
fn the_codec_string_of_an_nvenc_1080p_stream_is_in_the_high_tier() {
    let record = real_hvcc_record_named("nvenc_main_1920x1080");

    assert_eq!(
        hevc_codec_string(&record.hvcc),
        Some("hvc1.1.6.H123.90".to_string())
    );
}
