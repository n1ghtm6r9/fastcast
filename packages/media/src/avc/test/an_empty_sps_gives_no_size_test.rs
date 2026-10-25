use crate::avc::video_size_from_sps;

#[test]
fn an_empty_sps_gives_no_size() {
    assert_eq!(video_size_from_sps(&[]), None);
    assert_eq!(video_size_from_sps(&[0x67]), None);
}
