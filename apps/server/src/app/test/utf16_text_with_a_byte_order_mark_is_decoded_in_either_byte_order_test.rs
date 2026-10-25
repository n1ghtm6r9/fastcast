use super::utf16_bytes;
use crate::app::{UTF16_BE_BYTE_ORDER_MARK, UTF16_LE_BYTE_ORDER_MARK, decode_dotenv_text};

#[test]
fn utf16_text_with_a_byte_order_mark_is_decoded_in_either_byte_order() {
    let text = "SERVER_NAME='Игровой ПК 🎮'\r\n";
    let little_endian = [
        UTF16_LE_BYTE_ORDER_MARK.as_slice(),
        &utf16_bytes(text, u16::to_le_bytes),
    ]
    .concat();
    let big_endian = [
        UTF16_BE_BYTE_ORDER_MARK.as_slice(),
        &utf16_bytes(text, u16::to_be_bytes),
    ]
    .concat();

    assert_eq!(decode_dotenv_text(&little_endian).as_deref(), Some(text));
    assert_eq!(decode_dotenv_text(&big_endian).as_deref(), Some(text));
}
