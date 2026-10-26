use crate::gf::test::assert_kernels_agree;
use crate::test_support::random_bytes;

#[test]
fn every_available_kernel_agrees_with_the_scalar_kernel() {
    let lengths = [0, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 1369, 1370, 1371];
    let mut state = 0x2545_F491_4F6C_DD1D;
    let mut comparisons = 0;

    for length in lengths {
        for coefficient in 0..=255u8 {
            let source = random_bytes(&mut state, length);
            let initial = random_bytes(&mut state, length);

            comparisons += assert_kernels_agree(coefficient, &source, &initial, 0);
        }
    }

    assert!(comparisons >= lengths.len() * 256);
}
