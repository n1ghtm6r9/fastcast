use super::{TemporaryDirectory, utf16_bytes};
use crate::app::{UTF16_LE_BYTE_ORDER_MARK, load_dotenv_file};
use std::env;

#[test]
fn a_dotenv_in_utf16_with_a_byte_order_mark_is_loaded() {
    let directory = TemporaryDirectory::new("a_dotenv_in_utf16");
    let text = "FASTCAST_TEST_UTF16_NAME='Игровой ПК'\r\nFASTCAST_TEST_UTF16_FPS=120\r\n";
    let bytes = [
        UTF16_LE_BYTE_ORDER_MARK.as_slice(),
        &utf16_bytes(text, u16::to_le_bytes),
    ]
    .concat();
    let path = directory.write(".env", &bytes);

    load_dotenv_file(&path).expect("what PowerShell 5.1 writes with > must load");

    assert_eq!(
        env::var("FASTCAST_TEST_UTF16_NAME").as_deref(),
        Ok("Игровой ПК")
    );
    assert_eq!(env::var("FASTCAST_TEST_UTF16_FPS").as_deref(), Ok("120"));
}
