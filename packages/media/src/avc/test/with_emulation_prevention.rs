pub fn with_emulation_prevention(rbsp: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(rbsp.len());
    let mut zeros = 0;

    for &byte in rbsp {
        if zeros >= 2 && byte <= 3 {
            output.push(3);
            zeros = 0;
        }

        output.push(byte);
        zeros = if byte == 0 { zeros + 1 } else { 0 };
    }

    output
}
