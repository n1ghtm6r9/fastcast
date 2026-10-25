use super::real_hvcc_record_named;
use crate::hevc_codec_string;

#[test]
fn the_codec_string_of_an_nvenc_4k_stream_is_hvc1_1_6_l153_90() {
    let record = real_hvcc_record_named("nvenc_main_3840x2160");

    assert_eq!(
        hevc_codec_string(&record.hvcc),
        Some("hvc1.1.6.L153.90".to_string())
    );
}
