const EMULATION_PREVENTION_BYTE: u8 = 0x03;
const ZEROS_BEFORE_EMULATION_PREVENTION: usize = 2;

pub fn remove_emulation_prevention(data: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(data.len());
    let mut zeros = 0;

    for &byte in data {
        if zeros >= ZEROS_BEFORE_EMULATION_PREVENTION && byte == EMULATION_PREVENTION_BYTE {
            zeros = 0;
            continue;
        }

        output.push(byte);
        zeros = if byte == 0 { zeros + 1 } else { 0 };
    }

    output
}
