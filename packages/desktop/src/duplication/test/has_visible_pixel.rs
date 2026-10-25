pub fn has_visible_pixel(bgra: &[u8]) -> bool {
    bgra.as_chunks::<4>()
        .0
        .iter()
        .any(|pixel| pixel[..3] != [0, 0, 0])
}
