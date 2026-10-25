use super::{TEST_SEED, assert_close, gpu_planes, masked_cursor, random_bgra, visible_pointer};
use crate::colour::{BitDepth, reference_planes};
use crate::gpu::test::first_output_device;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn a_hidden_pointer_draws_nothing() {
    let device = first_output_device();
    let mut state = TEST_SEED;
    let bgra = random_bgra(&mut state, 64, 32);
    let mut pointer = visible_pointer(masked_cursor(), 10, 10, 1);
    pointer.visible = false;

    let planes = gpu_planes(&device, &bgra, 64, 32, BitDepth::Eight, &pointer);

    assert_close(
        &planes,
        &reference_planes(&bgra, 64, 32, BitDepth::Eight),
        1,
    );
}
