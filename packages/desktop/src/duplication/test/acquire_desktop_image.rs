use crate::duplication::{Duplication, FrameUpdate};
use crate::gpu::GpuDevice;
use std::time::{Duration, Instant};

pub fn acquire_desktop_image(device: &GpuDevice, duplication: &mut Duplication) -> FrameUpdate {
    let deadline = Instant::now() + Duration::from_secs(10);

    while Instant::now() < deadline {
        let update = duplication
            .acquire(device, Duration::from_millis(500))
            .unwrap();

        if let Some(update) = update.filter(|update| update.desktop) {
            return update;
        }
    }

    panic!("the desktop did not change for 10 seconds, so there is no image to capture");
}
