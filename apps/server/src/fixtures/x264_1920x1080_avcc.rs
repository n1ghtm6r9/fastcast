use crate::fixtures::hex_bytes;

const AVCC: &str = "0142c02affe100186742c02ada01e0089f970110000003001000000788f1832a01000468ce0fc8";

pub fn x264_1920x1080_avcc() -> Vec<u8> {
    hex_bytes(AVCC)
}
