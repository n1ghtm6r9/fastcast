use crate::colour::{BitDepth, Planes};
use crate::converter::planes_from_mapped;

#[test]
fn a_10_bit_layout_is_shifted_down_by_6() {
    let stored = |code: u16| code << 6;
    let mapped: Vec<u8> = [
        stored(64),
        stored(940),
        0xFFFF,
        0xFFFF,
        stored(100),
        stored(1023),
        0xFFFF,
        0xFFFF,
        stored(512),
        stored(960),
        0xFFFF,
        0xFFFF,
    ]
    .into_iter()
    .flat_map(u16::to_le_bytes)
    .collect();

    let planes = planes_from_mapped(&mapped, 8, 2, 2, BitDepth::Ten);

    assert_eq!(
        planes,
        Planes {
            width: 2,
            height: 2,
            luma: vec![64, 940, 100, 1023],
            chroma: vec![512, 960],
        }
    );
}
