use super::nal_units;

const HEVC_SPS: u8 = 33;
const EMULATION_PREVENTION_BYTE: u8 = 3;
const ZEROS_BEFORE_EMULATION_PREVENTION: usize = 2;

pub fn hevc_sps_rbsp(annex_b: &[u8]) -> Vec<u8> {
    let sps = nal_units(annex_b)
        .into_iter()
        .find(|unit| (unit[0] >> 1) & 0x3F == HEVC_SPS)
        .expect("an SPS NAL unit");
    let mut rbsp = Vec::with_capacity(sps.len());
    let mut zeros = 0;

    for &byte in sps {
        if zeros >= ZEROS_BEFORE_EMULATION_PREVENTION && byte == EMULATION_PREVENTION_BYTE {
            zeros = 0;
            continue;
        }

        zeros = if byte == 0 { zeros + 1 } else { 0 };
        rbsp.push(byte);
    }

    rbsp
}
