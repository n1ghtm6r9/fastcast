pub fn leb128_of(mut value: usize) -> Vec<u8> {
    let mut bytes = Vec::new();

    loop {
        let low_bits = (value & 0x7F) as u8;
        value >>= 7;

        if value == 0 {
            bytes.push(low_bits);
            return bytes;
        }

        bytes.push(low_bits | 0x80);
    }
}
