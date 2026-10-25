use super::device_named;
use crate::display::new_parsec_display;

#[test]
fn a_parsec_display_that_was_already_attached_is_not_new() {
    let physical = device_named("\\\\.\\DISPLAY1", "NVIDIA GeForce RTX 5070");
    let earlier = device_named("\\\\.\\DISPLAY10", "Parsec Virtual Display Adapter");
    let attached = vec![physical, earlier.clone()];
    let before = attached.clone();

    assert_eq!(new_parsec_display(&attached, &before), None);
}
