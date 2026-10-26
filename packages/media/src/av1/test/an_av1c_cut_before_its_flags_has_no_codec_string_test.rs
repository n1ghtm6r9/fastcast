use super::real_av1c_record_named;
use crate::av1_codec_string;

#[test]
fn an_av1c_cut_before_its_flags_has_no_codec_string() {
    let av1c = real_av1c_record_named("nvenc_main_1920x1080").av1c;

    assert_eq!(av1_codec_string(&av1c[..2]), None);
    assert_eq!(
        av1_codec_string(&av1c[..3]),
        Some("av01.0.09H.08".to_string())
    );
}
