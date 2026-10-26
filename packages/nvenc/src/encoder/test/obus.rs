pub fn obus(temporal_unit: &[u8]) -> Vec<(u8, &[u8])> {
    let mut obus = Vec::new();
    let mut rest = temporal_unit;

    while let [header, after_header @ ..] = rest {
        assert!(header & 0x02 != 0, "every OBU carries its size");

        let extension_length = usize::from(header & 0x04 != 0);
        let mut size = 0;
        let mut size_length = 0;

        loop {
            let byte = after_header[extension_length + size_length];
            size |= usize::from(byte & 0x7F) << (7 * size_length);
            size_length += 1;

            if byte & 0x80 == 0 {
                break;
            }
        }

        let payload_start = 1 + extension_length + size_length;
        obus.push((
            (header >> 3) & 0x0F,
            &rest[payload_start..payload_start + size],
        ));
        rest = &rest[payload_start + size..];
    }

    obus
}
