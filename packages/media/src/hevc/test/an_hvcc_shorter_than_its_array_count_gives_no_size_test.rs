use super::hvcc_header;
use crate::video_size_from_hvcc;

#[test]
fn an_hvcc_shorter_than_its_array_count_gives_no_size() {
    let hvcc = hvcc_header(0x01, 0x6000_0000, [0x90, 0, 0, 0, 0, 0], 120);

    assert_eq!(video_size_from_hvcc(&hvcc), None);
}
