pub fn random_bgra(state: &mut u64, width: usize, height: usize) -> Vec<u8> {
    (0..width * height * 4)
        .map(|_| {
            *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);

            let mut mixed = *state;
            mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);

            (mixed ^ (mixed >> 31)) as u8
        })
        .collect()
}
