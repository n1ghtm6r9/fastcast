use crate::app::{UTF16_BE_BYTE_ORDER_MARK, UTF16_LE_BYTE_ORDER_MARK, decode_dotenv_text};

#[test]
fn bytes_that_are_not_text_are_not_decoded() {
    let invalid_utf8 = b"SERVER_NAME=\xFF\xFE\n".to_vec();
    let half_a_utf16_unit = [UTF16_LE_BYTE_ORDER_MARK.as_slice(), b"S\0E"].concat();
    let lone_utf16_surrogate = [UTF16_BE_BYTE_ORDER_MARK.as_slice(), &[0xD8, 0x00]].concat();

    assert_eq!(decode_dotenv_text(&invalid_utf8), None);
    assert_eq!(decode_dotenv_text(&half_a_utf16_unit), None);
    assert_eq!(decode_dotenv_text(&lone_utf16_surrogate), None);
}
