use crate::colour::BitDepth;
use crate::converter::Converter;
use crate::gpu::test::first_output_device;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn a_converter_reports_the_depth_it_was_made_for() {
    let device = first_output_device();

    let converter = Converter::new(&device, 64, 32, BitDepth::Ten).unwrap();

    assert_eq!(converter.depth(), BitDepth::Ten);
}
