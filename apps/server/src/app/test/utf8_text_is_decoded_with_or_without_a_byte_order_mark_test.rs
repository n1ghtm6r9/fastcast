use crate::app::{UTF8_BYTE_ORDER_MARK, decode_dotenv_text};

#[test]
fn utf8_text_is_decoded_with_or_without_a_byte_order_mark() {
    let text = "SERVER_NAME='Игровой ПК'\n";
    let with_mark = [UTF8_BYTE_ORDER_MARK.as_slice(), text.as_bytes()].concat();

    assert_eq!(decode_dotenv_text(text.as_bytes()).as_deref(), Some(text));
    assert_eq!(decode_dotenv_text(&with_mark).as_deref(), Some(text));
}
