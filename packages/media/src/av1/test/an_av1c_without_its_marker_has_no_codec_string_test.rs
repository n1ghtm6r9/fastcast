use super::real_av1c_record_named;
use crate::av1_codec_string;

#[test]
fn an_av1c_without_its_marker_has_no_codec_string() {
    let mut av1c = real_av1c_record_named("nvenc_main_1920x1080").av1c;
    av1c[0] = 0x01;

    assert_eq!(av1_codec_string(&av1c), None);
}
