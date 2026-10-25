use crate::colour::BitDepth;
use crate::converter::{Converter, read_planes};
use crate::gpu::test::first_output_device;

#[test]
#[ignore = "needs a GPU and a desktop"]
#[should_panic(expected = "does not match the bit depth")]
fn a_texture_of_the_other_depth_is_refused() {
    let device = first_output_device();
    let converter = Converter::new(&device, 64, 32, BitDepth::Eight).unwrap();

    let _ = read_planes(&device, converter.output(), BitDepth::Ten);
}
