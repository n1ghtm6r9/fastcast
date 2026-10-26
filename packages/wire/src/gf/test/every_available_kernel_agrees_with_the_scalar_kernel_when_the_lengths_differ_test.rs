use crate::gf::test::assert_kernels_agree;
use crate::test_support::random_bytes;

#[test]
fn every_available_kernel_agrees_with_the_scalar_kernel_when_the_lengths_differ() {
    let length_pairs = [
        (100, 70),
        (70, 100),
        (0, 50),
        (50, 0),
        (33, 64),
        (64, 33),
        (1371, 1370),
        (1370, 1371),
    ];
    let mut state = 0xD1B5_4A32_D192_ED03;
    let mut compared = Vec::new();

    for (source_length, target_length) in length_pairs {
        for coefficient in 0..=255u8 {
            let source = random_bytes(&mut state, source_length);
            let initial = random_bytes(&mut state, target_length);

            compared = assert_kernels_agree(coefficient, &source, &initial, 0);
        }
    }

    assert!(compared.contains(&"scalar"));
}
