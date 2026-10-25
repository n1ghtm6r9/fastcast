pub fn hvcc_header(
    profile: u8,
    compatibility: u32,
    constraint_flags: [u8; 6],
    level: u8,
) -> Vec<u8> {
    let mut header = vec![1, profile];

    header.extend_from_slice(&compatibility.to_be_bytes());
    header.extend_from_slice(&constraint_flags);
    header.extend_from_slice(&[level, 0xf0, 0x00, 0xfc, 0xfd, 0xf8, 0xf8, 0x00, 0x00, 0x0f]);

    header
}
