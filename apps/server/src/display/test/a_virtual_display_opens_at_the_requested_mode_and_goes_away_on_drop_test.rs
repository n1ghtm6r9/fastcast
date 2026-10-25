use super::mode_of;
use crate::display::{POLL_INTERVAL, VirtualDisplay, attached_display_devices, poll_until};
use std::convert::Infallible;
use std::time::Duration;

fn is_attached(device_name: &str) -> bool {
    attached_display_devices()
        .iter()
        .any(|device| device.name == device_name)
}

#[test]
#[ignore = "adds a 4K monitor to this PC through the Parsec driver"]
fn a_virtual_display_opens_at_the_requested_mode_and_goes_away_on_drop() {
    let requested = mode_of(3840, 2160, 60);
    let display = VirtualDisplay::open(requested).expect("the virtual display opens");
    let device_name = display.device_name().to_owned();

    assert_eq!(display.mode(), requested);
    assert!(display.output_index() >= 1);
    assert!(is_attached(&device_name));

    drop(display);

    let gone = poll_until(Duration::from_secs(3), POLL_INTERVAL, || {
        Ok::<_, Infallible>((!is_attached(&device_name)).then_some(()))
    });

    assert_eq!(gone, Ok(Some(())));
}
