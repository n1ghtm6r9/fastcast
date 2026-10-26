use crate::gf::test::assert_kernels_agree;
use crate::test_support::random_bytes;

#[test]
fn every_available_kernel_agrees_with_the_scalar_kernel_on_unaligned_slices() {
    let lengths = [1, 17, 33, 100, 1370];
    let coefficients = [0, 1, 2, 3, 0x1D, 0x53, 0x8E, 0xCA, 0xFF];
    let mut state = 0x9E37_79B9_7F4A_7C15;
    let mut comparisons = 0;

    for length in lengths {
        for source_offset in 0..32 {
            let target_offset = (source_offset * 5) % 32;
            let source_buffer = random_bytes(&mut state, source_offset + length);
            let initial = random_bytes(&mut state, length);

            for coefficient in coefficients {
                comparisons += assert_kernels_agree(
                    coefficient,
                    &source_buffer[source_offset..],
                    &initial,
                    target_offset,
                );
            }
        }
    }

    assert!(comparisons >= lengths.len() * 32 * coefficients.len());
}
