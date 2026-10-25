use crate::display::text_of_wide;

#[test]
fn wide_text_ends_at_the_first_nul() {
    let padded: Vec<u16> = "\\\\.\\DISPLAY10"
        .encode_utf16()
        .chain([0, 0x41, 0x42])
        .collect();

    assert_eq!(text_of_wide(&padded), "\\\\.\\DISPLAY10");
    assert_eq!(text_of_wide(&[0x41, 0x42]), "AB");
    assert_eq!(text_of_wide(&[]), "");
}
