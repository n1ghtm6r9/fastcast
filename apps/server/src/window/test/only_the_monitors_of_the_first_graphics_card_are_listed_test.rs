use crate::window::monitors_of;
use crate::window::test::{output_on, two_monitors};

#[test]
fn only_the_monitors_of_the_first_graphics_card_are_listed() {
    let outputs = [
        output_on(0, 0, "\\\\.\\DISPLAY1", 2560, 1440),
        output_on(1, 0, "\\\\.\\DISPLAY7", 3840, 2160),
        output_on(0, 1, "\\\\.\\DISPLAY2", 1920, 1080),
    ];

    assert_eq!(monitors_of(&outputs), two_monitors());
}
