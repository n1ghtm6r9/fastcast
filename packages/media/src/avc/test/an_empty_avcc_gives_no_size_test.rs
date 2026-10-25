use crate::video_size_from_avcc;

#[test]
fn an_empty_avcc_gives_no_size() {
    assert_eq!(video_size_from_avcc(&[]), None);
}
