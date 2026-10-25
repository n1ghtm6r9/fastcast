use crate::video_size_from_hvcc;

#[test]
fn an_empty_hvcc_gives_no_size() {
    assert_eq!(video_size_from_hvcc(&[]), None);
}
