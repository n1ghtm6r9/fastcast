use crate::colour::BitDepth;
use crate::converter::output_scale;

#[test]
fn every_8_bit_code_is_stored_as_itself() {
    let scale = output_scale(BitDepth::Eight);

    for code in 0..=255u32 {
        let stored = (code as f32 * scale * 255.0).round() as u32;

        assert_eq!(stored, code);
    }
}
