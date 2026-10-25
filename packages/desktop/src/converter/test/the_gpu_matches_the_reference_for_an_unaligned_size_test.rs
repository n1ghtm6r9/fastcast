use super::{TEST_SEED, assert_close, gpu_planes, random_bgra};
use crate::colour::{BitDepth, reference_planes};
use crate::duplication::PointerState;
use crate::gpu::test::first_output_device;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn the_gpu_matches_the_reference_for_an_unaligned_size() {
    let device = first_output_device();
    let mut state = TEST_SEED;
    let bgra = random_bgra(&mut state, 66, 34);

    let planes = gpu_planes(
        &device,
        &bgra,
        66,
        34,
        BitDepth::Eight,
        &PointerState::default(),
    );

    assert_close(
        &planes,
        &reference_planes(&bgra, 66, 34, BitDepth::Eight),
        1,
    );
}
