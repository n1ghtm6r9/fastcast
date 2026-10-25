use super::{
    TEST_SEED, assert_close, blended_cursor, gpu_planes, random_bgra, reference_with_cursor,
    visible_pointer,
};
use crate::colour::BitDepth;
use crate::gpu::test::first_output_device;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn a_cursor_over_the_far_corner_is_clipped_like_the_reference() {
    let device = first_output_device();
    let mut state = TEST_SEED;
    let bgra = random_bgra(&mut state, 64, 32);
    let cursor = blended_cursor();
    let pointer = visible_pointer(cursor.clone(), 60, 28, 1);

    let planes = gpu_planes(&device, &bgra, 64, 32, BitDepth::Eight, &pointer);

    assert_close(
        &planes,
        &reference_with_cursor(&bgra, 64, 32, BitDepth::Eight, &cursor, 60, 28),
        2,
    );
}
