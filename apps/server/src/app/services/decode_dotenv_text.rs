use crate::app::{
    UTF8_BYTE_ORDER_MARK, UTF16_BE_BYTE_ORDER_MARK, UTF16_LE_BYTE_ORDER_MARK, decode_utf16_text,
};

pub fn decode_dotenv_text(bytes: &[u8]) -> Option<String> {
    if let Some(body) = bytes.strip_prefix(&UTF16_LE_BYTE_ORDER_MARK) {
        return decode_utf16_text(body, u16::from_le_bytes);
    }

    if let Some(body) = bytes.strip_prefix(&UTF16_BE_BYTE_ORDER_MARK) {
        return decode_utf16_text(body, u16::from_be_bytes);
    }

    let body = bytes.strip_prefix(&UTF8_BYTE_ORDER_MARK).unwrap_or(bytes);

    str::from_utf8(body).ok().map(str::to_owned)
}
