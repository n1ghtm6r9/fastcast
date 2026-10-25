use super::device_named;

#[test]
fn the_parsec_adapter_is_recognised_in_any_letter_case() {
    assert!(device_named("\\\\.\\DISPLAY10", "PARSEC virtual display adapter").is_parsec());
    assert!(device_named("\\\\.\\DISPLAY10", "Parsec Virtual Display Adapter").is_parsec());
    assert!(!device_named("\\\\.\\DISPLAY1", "NVIDIA GeForce RTX 5070").is_parsec());
}
