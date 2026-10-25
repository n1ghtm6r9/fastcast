pub fn bit_string_bytes(bits: &str) -> Vec<u8> {
    let digits: Vec<bool> = bits
        .chars()
        .filter(|character| !character.is_whitespace())
        .map(|character| character == '1')
        .collect();

    digits
        .chunks(8)
        .map(|chunk| {
            chunk
                .iter()
                .enumerate()
                .filter(|(_, bit)| **bit)
                .fold(0u8, |byte, (position, _)| byte | (0x80 >> position))
        })
        .collect()
}
