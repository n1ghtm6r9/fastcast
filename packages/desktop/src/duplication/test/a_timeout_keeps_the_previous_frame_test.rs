use super::{acquire_desktop_image, frame_bytes, has_visible_pixel};
use crate::duplication::Duplication;
use crate::gpu::GpuDevice;
use crate::gpu::test::first_attached_output;
use std::time::Duration;
use windows::core::Interface;

#[test]
#[ignore = "needs a GPU and a desktop that is updating"]
fn a_timeout_keeps_the_previous_frame() {
    let output = first_attached_output();
    let device = GpuDevice::new(output.adapter_luid).unwrap();
    let mut duplication = Duplication::new(&device, &output).unwrap();
    acquire_desktop_image(&device, &mut duplication);
    let frame = duplication.frame().as_raw();

    for _ in 0..50 {
        let before = frame_bytes(&device, duplication.frame());
        let update = duplication
            .acquire(&device, Duration::from_millis(10))
            .unwrap();

        if update.is_none() {
            assert_eq!(frame_bytes(&device, duplication.frame()), before);
            break;
        }
    }

    let bytes = frame_bytes(&device, duplication.frame());

    assert_eq!(duplication.frame().as_raw(), frame);
    assert!(has_visible_pixel(&bytes));
}
