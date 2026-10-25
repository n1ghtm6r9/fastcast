use super::hvcc_header;
use crate::hevc_codec_string;

#[test]
fn the_level_is_written_in_decimal() {
    let level_1 = hvcc_header(0x01, 0x6000_0000, [0; 6], 30);
    let level_5_1 = hvcc_header(0x01, 0x6000_0000, [0; 6], 153);
    let level_0 = hvcc_header(0x01, 0x6000_0000, [0; 6], 0);

    assert_eq!(
        hevc_codec_string(&level_1),
        Some("hvc1.1.6.L30".to_string())
    );
    assert_eq!(
        hevc_codec_string(&level_5_1),
        Some("hvc1.1.6.L153".to_string())
    );
    assert_eq!(hevc_codec_string(&level_0), Some("hvc1.1.6.L0".to_string()));
}
