use crate::colour::BitDepth;
use crate::converter::Converter;
use crate::gpu::DesktopError;
use crate::gpu::test::first_output_device;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn odd_sizes_are_rejected() {
    let device = first_output_device();

    let result = Converter::new(&device, 63, 32, BitDepth::Eight);

    assert!(matches!(
        result,
        Err(DesktopError::OddSize {
            width: 63,
            height: 32
        })
    ));
}
