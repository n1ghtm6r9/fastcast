use super::device_named;
use crate::display::new_parsec_display;

#[test]
fn a_parsec_display_that_was_not_attached_before_is_the_new_one() {
    let physical = device_named("\\\\.\\DISPLAY1", "NVIDIA GeForce RTX 5070");
    let virtual_one = device_named("\\\\.\\DISPLAY10", "Parsec Virtual Display Adapter");
    let before = vec![physical.clone()];
    let attached = vec![physical, virtual_one.clone()];

    assert_eq!(new_parsec_display(&attached, &before), Some(&virtual_one));
}
