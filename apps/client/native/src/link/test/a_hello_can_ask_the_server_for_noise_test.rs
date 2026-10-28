use crate::link::hello_of;

#[test]
fn a_hello_can_ask_the_server_for_noise() {
    assert!(hello_of(7, 2560, 1440, 120, &[2, 10], true).noise);
    assert!(!hello_of(7, 2560, 1440, 120, &[2, 10], false).noise);
}
