use super::hvcc_header;
use crate::hevc_codec_string;

#[test]
fn the_compatibility_flags_are_written_in_reverse_bit_order() {
    let main = hvcc_header(0x01, 0x6000_0000, [0; 6], 93);
    let main_10 = hvcc_header(0x02, 0x2000_0000, [0; 6], 93);
    let range_extensions = hvcc_header(0x04, 0x0800_0000, [0; 6], 93);
    let lowest_flags = hvcc_header(0x01, 0x0000_0003, [0; 6], 93);

    assert_eq!(hevc_codec_string(&main), Some("hvc1.1.6.L93".to_string()));
    assert_eq!(
        hevc_codec_string(&main_10),
        Some("hvc1.2.4.L93".to_string())
    );
    assert_eq!(
        hevc_codec_string(&range_extensions),
        Some("hvc1.4.10.L93".to_string())
    );
    assert_eq!(
        hevc_codec_string(&lowest_flags),
        Some("hvc1.1.C0000000.L93".to_string())
    );
}
