use crate::link::hello_of;

#[test]
fn impossible_hello_values_are_left_out() {
    let hello = hello_of(7, -1, 70_000, 120, &[9, 8, 2], false);

    assert_eq!(hello.width, 0);
    assert_eq!(hello.height, 0);
    assert_eq!(hello.fps, 120);
    assert!(hello.codecs.is_empty());
}
