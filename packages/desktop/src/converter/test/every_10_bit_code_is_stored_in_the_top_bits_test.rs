use crate::colour::BitDepth;
use crate::converter::output_scale;

#[test]
fn every_10_bit_code_is_stored_in_the_top_bits() {
    let scale = output_scale(BitDepth::Ten);

    for code in 0..=1023u32 {
        let stored = (code as f32 * scale * 65535.0).round() as u32;

        assert_eq!(stored, code << 6);
    }
}
