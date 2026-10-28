use super::acquire_desktop_image;
use crate::duplication::{Duplication, present_age};
use crate::gpu::GpuDevice;
use crate::gpu::test::first_attached_output;
use std::time::Duration;

#[test]
#[ignore = "needs a GPU and a desktop that is updating"]
fn a_desktop_image_tells_when_the_screen_showed_it() {
    let output = first_attached_output();
    let device = GpuDevice::new(output.adapter_luid).unwrap();
    let mut duplication = Duplication::new(&device, &output).unwrap();

    let update = acquire_desktop_image(&device, &mut duplication);

    let age = present_age(update.presented).expect("a desktop image has a present time");
    assert!(age < Duration::from_secs(1), "{age:?}");
}
