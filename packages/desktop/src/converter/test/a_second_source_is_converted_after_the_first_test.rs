use super::{TEST_SEED, assert_close, bgra_texture, random_bgra};
use crate::colour::{BitDepth, reference_planes};
use crate::converter::{Converter, read_planes};
use crate::duplication::PointerState;
use crate::gpu::test::first_output_device;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn a_second_source_is_converted_after_the_first() {
    let device = first_output_device();
    let mut state = TEST_SEED;
    let first = random_bgra(&mut state, 64, 32);
    let second = random_bgra(&mut state, 64, 32);
    let first_source = bgra_texture(&device, 64, 32, &first);
    let second_source = bgra_texture(&device, 64, 32, &second);
    let mut converter = Converter::new(&device, 64, 32, BitDepth::Eight).unwrap();
    let pointer = PointerState::default();

    converter.convert(&device, &first_source, &pointer).unwrap();
    converter
        .convert(&device, &second_source, &pointer)
        .unwrap();
    let planes = read_planes(&device, converter.output(), BitDepth::Eight).unwrap();

    assert_close(
        &planes,
        &reference_planes(&second, 64, 32, BitDepth::Eight),
        1,
    );
}
