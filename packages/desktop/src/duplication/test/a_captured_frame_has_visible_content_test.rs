use super::{acquire_desktop_image, frame_bytes, has_visible_pixel};
use crate::duplication::Duplication;
use crate::gpu::GpuDevice;
use crate::gpu::test::first_attached_output;

#[test]
#[ignore = "needs a GPU and a desktop that is updating"]
fn a_captured_frame_has_visible_content() {
    let output = first_attached_output();
    let device = GpuDevice::new(output.adapter_luid).unwrap();
    let mut duplication = Duplication::new(&device, &output).unwrap();
    acquire_desktop_image(&device, &mut duplication);

    let bytes = frame_bytes(&device, duplication.frame());

    assert!(has_visible_pixel(&bytes));
}
