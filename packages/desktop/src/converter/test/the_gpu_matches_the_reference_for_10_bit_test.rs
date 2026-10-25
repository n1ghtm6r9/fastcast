use super::{TEST_SEED, assert_close, gpu_planes, random_bgra};
use crate::colour::{BitDepth, reference_planes};
use crate::duplication::PointerState;
use crate::gpu::test::first_output_device;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn the_gpu_matches_the_reference_for_10_bit() {
    let device = first_output_device();
    let mut state = TEST_SEED;
    let mut bgra = random_bgra(&mut state, 64, 32);
    bgra[..16].copy_from_slice(&[
        0, 0, 255, 255, 0, 255, 0, 255, 255, 0, 0, 255, 255, 255, 255, 255,
    ]);

    let planes = gpu_planes(
        &device,
        &bgra,
        64,
        32,
        BitDepth::Ten,
        &PointerState::default(),
    );

    assert_close(&planes, &reference_planes(&bgra, 64, 32, BitDepth::Ten), 1);
}
