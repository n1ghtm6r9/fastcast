use crate::hevc::video_size_from_hevc_sps;

#[test]
fn an_empty_sps_gives_no_size() {
    assert_eq!(video_size_from_hevc_sps(&[]), None);
    assert_eq!(video_size_from_hevc_sps(&[0x42]), None);
    assert_eq!(video_size_from_hevc_sps(&[0x42, 0x01]), None);
}
