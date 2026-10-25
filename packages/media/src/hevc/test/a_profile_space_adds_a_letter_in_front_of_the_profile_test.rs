use super::hvcc_header;
use crate::hevc_codec_string;

#[test]
fn a_profile_space_adds_a_letter_in_front_of_the_profile() {
    for (profile, expected) in [
        (0x41, "hvc1.A1.6.L93"),
        (0x81, "hvc1.B1.6.L93"),
        (0xc1, "hvc1.C1.6.L93"),
    ] {
        let hvcc = hvcc_header(profile, 0x6000_0000, [0; 6], 93);

        assert_eq!(hevc_codec_string(&hvcc), Some(expected.to_string()));
    }
}
