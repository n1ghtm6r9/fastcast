use super::{
    TEST_SEED, assert_close, bgra_texture, blended_cursor, masked_cursor, random_bgra,
    reference_with_cursor, visible_pointer,
};
use crate::colour::BitDepth;
use crate::converter::{Converter, read_planes};
use crate::gpu::test::first_output_device;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn a_new_pointer_shape_replaces_the_old_one() {
    let device = first_output_device();
    let mut state = TEST_SEED;
    let bgra = random_bgra(&mut state, 64, 32);
    let source = bgra_texture(&device, 64, 32, &bgra);
    let mut converter = Converter::new(&device, 64, 32, BitDepth::Eight).unwrap();
    let replacement = blended_cursor();

    converter
        .convert(
            &device,
            &source,
            &visible_pointer(masked_cursor(), 10, 10, 1),
        )
        .unwrap();
    converter
        .convert(
            &device,
            &source,
            &visible_pointer(replacement.clone(), 10, 10, 2),
        )
        .unwrap();
    let planes = read_planes(&device, converter.output(), BitDepth::Eight).unwrap();

    assert_close(
        &planes,
        &reference_with_cursor(&bgra, 64, 32, BitDepth::Eight, &replacement, 10, 10),
        2,
    );
}
