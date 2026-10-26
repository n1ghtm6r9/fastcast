use crate::fixtures::hex_bytes;

const AV1C: &str = "810e4c000a0f00000072efbfe1bc0219d020201040";

pub fn nvenc_3840x2160_av1c() -> Vec<u8> {
    hex_bytes(AV1C)
}
