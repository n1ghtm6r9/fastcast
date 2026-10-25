use super::{
    TEST_SEED, assert_close, gpu_planes, masked_cursor, random_bgra, reference_with_cursor,
    visible_pointer,
};
use crate::colour::BitDepth;
use crate::gpu::test::first_output_device;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn an_inverting_cursor_is_composited_like_the_reference() {
    let device = first_output_device();
    let mut state = TEST_SEED;
    let bgra = random_bgra(&mut state, 64, 32);
    let cursor = masked_cursor();
    let pointer = visible_pointer(cursor.clone(), -3, 5, 1);

    let planes = gpu_planes(&device, &bgra, 64, 32, BitDepth::Eight, &pointer);

    assert_close(
        &planes,
        &reference_with_cursor(&bgra, 64, 32, BitDepth::Eight, &cursor, -3, 5),
        1,
    );
}
