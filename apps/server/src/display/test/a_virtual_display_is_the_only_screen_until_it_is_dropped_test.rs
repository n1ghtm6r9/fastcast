use super::mode_of;
use crate::display::{
    POLL_INTERVAL, VirtualDisplay, attached_display_devices, current_placements, layout_is_back,
    poll_until,
};
use std::convert::Infallible;
use std::time::Duration;

fn attached_names() -> Vec<String> {
    attached_display_devices()
        .into_iter()
        .map(|device| device.name)
        .collect()
}

#[test]
#[ignore = "turns the monitors of this PC off for a few seconds and shows a 4K Parsec display instead"]
fn a_virtual_display_is_the_only_screen_until_it_is_dropped() {
    let before = current_placements();
    let requested = mode_of(3840, 2160, 60);
    let display = VirtualDisplay::open(requested).expect("the virtual display opens");
    let device_name = display.device_name().to_owned();

    assert_eq!(display.mode(), requested);
    assert_eq!(display.output_index(), 0);
    assert_eq!(attached_names(), vec![device_name.clone()]);

    drop(display);

    let gone = poll_until(Duration::from_secs(3), POLL_INTERVAL, || {
        Ok::<_, Infallible>((!attached_names().contains(&device_name)).then_some(()))
    });

    assert_eq!(gone, Ok(Some(())));
    assert!(layout_is_back(&before, &current_placements()));
}
