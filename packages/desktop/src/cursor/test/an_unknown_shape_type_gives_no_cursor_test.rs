use crate::decode_cursor_shape;

#[test]
fn an_unknown_shape_type_gives_no_cursor() {
    assert_eq!(decode_cursor_shape(3, 1, 1, 4, &[0, 0, 0, 0]), None);
}
