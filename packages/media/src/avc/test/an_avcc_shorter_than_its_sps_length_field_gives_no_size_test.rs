use crate::video_size_from_avcc;

#[test]
fn an_avcc_shorter_than_its_sps_length_field_gives_no_size() {
    let avcc = [1, 100, 0, 42, 0xff, 0xe1, 0x00];

    assert_eq!(video_size_from_avcc(&avcc), None);
}
