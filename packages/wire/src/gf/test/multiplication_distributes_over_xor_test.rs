use crate::gf::gf_mul;

#[test]
fn multiplication_distributes_over_xor() {
    for a in 0..=255u8 {
        for b in 0..=255u8 {
            for c in [1, 2, 0x53, 0xCA, 255] {
                assert_eq!(gf_mul(a ^ b, c), gf_mul(a, c) ^ gf_mul(b, c));
            }
        }
    }
}
