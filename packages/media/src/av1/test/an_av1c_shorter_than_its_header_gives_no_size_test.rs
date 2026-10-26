use crate::video_size_from_av1c;

#[test]
fn an_av1c_shorter_than_its_header_gives_no_size() {
    assert_eq!(video_size_from_av1c(&[0x81, 0x08]), None);
}
