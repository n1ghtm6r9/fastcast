pub fn hex_bytes(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|start| {
            u8::from_str_radix(&text[start..start + 2], 16).expect("the text is hexadecimal")
        })
        .collect()
}
