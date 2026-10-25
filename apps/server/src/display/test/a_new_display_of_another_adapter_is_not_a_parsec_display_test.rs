use super::device_named;
use crate::display::new_parsec_display;

#[test]
fn a_new_display_of_another_adapter_is_not_a_parsec_display() {
    let physical = device_named("\\\\.\\DISPLAY1", "NVIDIA GeForce RTX 5070");
    let other = device_named("\\\\.\\DISPLAY2", "Intel(R) UHD Graphics");
    let before = vec![physical.clone()];
    let attached = vec![physical, other];

    assert_eq!(new_parsec_display(&attached, &before), None);
}
