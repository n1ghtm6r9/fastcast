use crate::colour::{BitDepth, Planes};
use crate::converter::planes_from_mapped;

#[test]
fn an_8_bit_layout_is_read_plane_by_plane() {
    let first_luma_row = [1, 2, 3, 4, 90, 91];
    let second_luma_row = [5, 6, 7, 8, 92, 93];
    let chroma_row = [11, 12, 13, 14, 94, 95];
    let mapped = [first_luma_row, second_luma_row, chroma_row].concat();

    let planes = planes_from_mapped(&mapped, 6, 4, 2, BitDepth::Eight);

    assert_eq!(
        planes,
        Planes {
            width: 4,
            height: 2,
            luma: vec![1, 2, 3, 4, 5, 6, 7, 8],
            chroma: vec![11, 12, 13, 14],
        }
    );
}
