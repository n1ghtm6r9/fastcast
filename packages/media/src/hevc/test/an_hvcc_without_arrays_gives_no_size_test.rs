use super::hvcc_with_arrays;
use crate::video_size_from_hvcc;

#[test]
fn an_hvcc_without_arrays_gives_no_size() {
    assert_eq!(video_size_from_hvcc(&hvcc_with_arrays(&[])), None);
}
