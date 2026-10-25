use crate::video_size_from_avcc;

#[test]
fn an_avcc_without_any_sps_gives_no_size() {
    let avcc = [1, 100, 0, 42, 0xff, 0xe0, 0];

    assert_eq!(video_size_from_avcc(&avcc), None);
}
