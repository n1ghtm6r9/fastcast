use crate::display::{DisplayError, PARSEC_VDD_DOWNLOAD_URL};

#[test]
fn the_missing_driver_error_says_where_to_get_it() {
    let message = DisplayError::DriverMissing.to_string();

    assert!(message.contains("Parsec Virtual Display Adapter"));
    assert!(message.contains(PARSEC_VDD_DOWNLOAD_URL));
}
